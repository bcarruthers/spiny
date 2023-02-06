use std::ops::Range;

use wgpu::util::DeviceExt;

use glam::*;
use indexmap::{map::Entry, IndexMap, IndexSet};

pub fn calc_uniform_alignment_size<T>(device: &wgpu::Device) -> wgpu::BufferAddress {
    // Make the `uniform_alignment` >= `entity_uniform_size` and aligned to `min_uniform_buffer_offset_alignment`.
    // Note: dynamic uniform offsets also have to be aligned to `Limits::min_uniform_buffer_offset_alignment`.
    let size = std::mem::size_of::<T>() as wgpu::BufferAddress;
    let alignment =
        device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
    let rem = size % alignment;
    if rem != 0 {
        size + alignment - rem
    } else {
        size
    }
}

#[derive(Clone)]
struct Slot(u32);

struct FixedSizeBufferPool {
    label: String,
    size: usize,
    count: usize,
    buffers: Vec<wgpu::Buffer>,
}

impl FixedSizeBufferPool {
    fn new(size: usize, label: &str) -> Self {
        Self {
            label: label.to_string(),
            size,
            count: 0,
            buffers: Vec::new(),
        }
    }

    fn pop(&mut self, device: &wgpu::Device) -> wgpu::Buffer {
        if let Some(buffer) = self.buffers.pop() {
            buffer
        } else {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{}{}", self.label, self.count)),
                size: self.size as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: true,
            })
        }
    }

    fn push(&mut self, buffer: wgpu::Buffer) {
        self.buffers.push(buffer)
    }
}

struct Page {
    buffer: wgpu::Buffer,
    count: u32,
    free: Vec<Slot>,
}

impl Page {
    fn new(buffer: wgpu::Buffer) -> Self {
        Self {
            buffer,
            count: 0,
            free: Vec::new(),
        }
    }

    /// Returns true if page is fully used
    fn allocate(&mut self, max_alloc: u32) -> (Slot, bool) {
        let slot = if let Some(slot) = self.free.pop() {
            slot
        } else {
            let slot = Slot(self.count);
            self.count += 1;
            slot
        };
        let filled = self.count == max_alloc && self.free.is_empty();
        (slot, filled)
    }

    /// Returns true if page is fully free
    fn deallocate(&mut self, slot: Slot) -> bool {
        self.free.push(slot);
        self.free.len() == self.count as usize
    }
}

struct IdPool {
    pool: Vec<u32>,
    count: u32,
}

impl IdPool {
    const UNDEFINED_ID: u32 = 0;

    fn new() -> Self {
        Self {
            pool: Vec::new(),
            count: Self::UNDEFINED_ID,
        }
    }

    fn pop(&mut self) -> u32 {
        if let Some(id) = self.pool.pop() {
            id
        } else {
            self.count += 1;
            self.count
        }
    }

    fn push(&mut self, id: u32) {
        self.pool.push(id);
    }
}

struct AllocationPool {
    pages: IndexMap<u32, Page>,
    free: IndexSet<u32>,
    id_pool: IdPool,
}

impl AllocationPool {
    fn new() -> Self {
        Self {
            pages: IndexMap::new(),
            free: IndexSet::new(),
            id_pool: IdPool::new(),
        }
    }

    fn allocate(
        &mut self,
        max_alloc: u32,
        buffer_pool: &mut FixedSizeBufferPool,
        device: &wgpu::Device,
    ) -> (u32, Slot, &wgpu::Buffer) {
        if self.free.is_empty() {
            // If no pages with empty slots, take a new page
            let id = self.id_pool.pop();
            self.pages.insert(id, Page::new(buffer_pool.pop(device)));
            self.free.insert(id);
        }
        // Allocate a slot from the first free page
        let page_id = *self.free.iter().next().unwrap();
        let page = self.pages.get_mut(&page_id).unwrap();
        let (slot, filled) = page.allocate(max_alloc);
        // If page is now filled, remove it from free set
        if filled {
            self.free.remove(&page_id);
        }
        (page_id, slot, &mut page.buffer)
    }

    fn deallocate(&mut self, page_id: u32, slot: Slot, buffer_pool: &mut FixedSizeBufferPool) {
        match self.pages.entry(page_id) {
            Entry::Vacant(_) => panic!("Entry {} missing", page_id),
            Entry::Occupied(mut entry) => {
                if entry.get_mut().deallocate(slot) {
                    let page = entry.remove();
                    buffer_pool.push(page.buffer);
                    self.id_pool.push(page_id);
                }
            }
        }
    }

    fn get_buffer(&self, page_id: u32, range: Range<wgpu::BufferAddress>) -> wgpu::BufferSlice {
        let page = self.pages.get(&page_id).unwrap();
        page.buffer.slice(range)
    }

    // fn draw<'a>(&self, page_id: u32, range: Range<wgpu::BufferAddress>, render_pass: &'a mut wgpu::RenderPass<'a>) {
    //     let page = self.pages.get(&page_id).unwrap();
    //     let len = range.end - range.start;
    //     render_pass.set_vertex_buffer(0, page.buffer.slice(range));
    //     render_pass.draw(0..len as u32, 0..1);
    // }
}

fn create_quad_indices(quad_count: usize) -> Vec<u32> {
    let mut indices = Vec::new();
    for i in 0..quad_count {
        let vi = i as u32 * 4;
        indices.push(vi + 0);
        indices.push(vi + 1);
        indices.push(vi + 2);
        indices.push(vi + 0);
        indices.push(vi + 2);
        indices.push(vi + 3);
    }
    indices
}

fn create_index_buffer(device: &wgpu::Device, quad_count: usize) -> wgpu::Buffer {
    let indices = create_quad_indices(quad_count);
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sprite_index_buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    })
}

pub struct QuadBuffer {
    index_buffer: wgpu::Buffer,
    quad_capacity: u32,
    index_count: u32,
}

impl QuadBuffer {
    pub fn new(device: &wgpu::Device, quad_count: u32) -> Self {
        let index_buffer = create_index_buffer(device, quad_count as usize);
        Self {
            index_buffer,
            quad_capacity: quad_count,
            index_count: 0,
        }
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.index_buffer.slice(..)
    }

    pub fn indices(&self) -> Range<u32> {
        0..self.index_count
    }

    pub fn update(&mut self, device: &wgpu::Device, quad_count: u32) {
        if quad_count > self.quad_capacity {
            let new_capacity = quad_count.next_power_of_two();
            self.index_buffer = create_index_buffer(device, new_capacity as usize);
            self.quad_capacity = new_capacity;
        }
        self.index_count = quad_count * 6;
    }
}

#[derive(Clone)]
pub struct Handle {
    pool: u32,
    page: u32,
    slot: Slot,
    len: u32,
}

impl Handle {
    pub const UNDEFINED: Handle = Handle {
        pool: 0,
        page: IdPool::UNDEFINED_ID,
        slot: Slot(0),
        len: 0,
    };

    pub fn len(&self) -> u32 {
        self.len
    }
}

pub struct BufferPool {
    min_size_pow: u32,
    buffers: FixedSizeBufferPool,
    pools: Vec<AllocationPool>,
}

impl BufferPool {
    fn next_log2(n: u64) -> u32 {
        if n <= 1 {
            0
        } else {
            64 - (n - 1).leading_zeros()
        }
    }

    pub fn new(min_size_pow: u32, max_size_pow: u32, label: &str) -> Self {
        let size_count = max_size_pow - min_size_pow + 1;
        let max_size = 1usize << max_size_pow;
        Self {
            min_size_pow,
            buffers: FixedSizeBufferPool::new(max_size, label),
            pools: (0..size_count)
                .into_iter()
                .map(|_| AllocationPool::new())
                .collect::<Vec<_>>(),
        }
    }

    pub fn allocate(&mut self, data: &[u8], device: &wgpu::Device, queue: &wgpu::Queue) -> Handle {
        // Truncate if data is larger than pool allows
        let max_size_pow = self.min_size_pow + self.pools.len() as u32 - 1;
        let size_pow = Self::next_log2(data.len() as u64).clamp(self.min_size_pow, max_size_pow);
        let size = data.len().min(1 << size_pow);
        let truncated = &data[0..size];
        // Get pool for this size allocation
        let size_index = size_pow - self.min_size_pow;
        let pool = &mut self.pools[size_index as usize];
        // Calc how many allocations of this size can fit into a page
        let max_alloc = 1 << (max_size_pow - size_pow);
        // Allocate block
        let (page_id, slot, buffer) = pool.allocate(max_alloc, &mut self.buffers, &device);
        let offset = slot.0 << size_pow;
        queue.write_buffer(buffer, offset as u64, truncated);
        Handle {
            pool: size_index,
            page: page_id,
            slot,
            len: size as u32,
        }
    }

    pub fn deallocate(&mut self, handle: Handle) {
        let pool = &mut self.pools[handle.pool as usize];
        pool.deallocate(handle.page, handle.slot, &mut self.buffers)
    }

    pub fn get_buffer(&self, handle: &Handle) -> wgpu::BufferSlice {
        let pool = &self.pools[handle.pool as usize];
        let size_pow = handle.pool + self.min_size_pow;
        let start = (handle.slot.0 as u64) << size_pow;
        let end = start + handle.len as u64;
        pool.get_buffer(handle.page, start..end)
    }
}
