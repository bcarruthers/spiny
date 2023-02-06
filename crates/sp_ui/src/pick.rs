use glam::Vec2;
use sp_math::range::Range2;

pub fn is_point_in_triangle(v: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> bool {
    let a = 0.5f32 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
    let sign = if a < 0.0f32 { -1.0f32 } else { 1.0f32 };
    let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * v.x + (p0.x - p2.x) * v.y) * sign;
    let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * v.x + (p1.x - p0.x) * v.y) * sign;
    s >= 0.0f32 && t >= 0.0f32 && (s + t) <= 2.0f32 * a * sign
}

/// Returns index of first vertex contained in rect
pub fn pick_point(verts: &[Vec2], rect: Range2) -> Option<usize> {
    for i in 0..verts.len() {
        if rect.contains(verts[i]) {
            return Some(i);
        }
    }
    None
}

/// Returns index of first triangle containing point
pub fn pick_triangle(verts: &[Vec2], p: Vec2) -> Option<usize> {
    for i in 0..verts.len() / 3 {
        let vi = i * 3;
        let v0 = verts[vi + 0];
        let v1 = verts[vi + 1];
        let v2 = verts[vi + 2];
        if is_point_in_triangle(p, v0, v1, v2) {
            return Some(i);
        }
    }
    None
}

/// Returns index of first quad containing point
pub fn pick_quad(verts: &[Vec2], p: Vec2) -> Option<usize> {
    for i in 0..verts.len() / 4 {
        let vi = i * 4;
        let v0 = verts[vi + 0];
        let v1 = verts[vi + 1];
        let v2 = verts[vi + 2];
        let v3 = verts[vi + 3];
        if is_point_in_triangle(p, v0, v1, v2) || is_point_in_triangle(p, v0, v2, v3) {
            return Some(i);
        }
    }
    None
}

// [<Struct>]
// type PickLayerDescriptor = {
//     LayerId : int
//     CameraId : int
//     Primitive : SpritePrimitive
//     FlushMode : SpriteFlushMode
//     }

// [<Struct>]
// type PickResult = {
//     LayerId : int
//     PrimitiveIndex : int
//     WorldPosition : Vec2
//     }

// [<Struct>]
// type PickResult<'a> = {
//     Param : 'a
//     LayerId : int
//     PrimitiveIndex : int
//     }

// type IPickLayer =
//     abstract Descriptor : PickLayerDescriptor
//     abstract WrittenVertexSpan : ReadOnlySpan<Vec2>
//     abstract Clear : unit -> unit

// type PickLayer<'a>(desc : PickLayerDescriptor) =
//     let values = ArrayBufferWriter<'a>()
//     let vertices = ArrayBufferWriter<Vec2>()
//     member c.Descriptor = desc
//     member c.Values = values :> IBufferWriter<'a>
//     member c.VertexWriter = vertices :> IBufferWriter<Vec2>
//     member c.WrittenValueSpan = values.WrittenSpan
//     member c.WrittenVertexSpan = vertices.WrittenSpan
//     member c.Clear() =
//         values.Clear()
//         vertices.Clear()
//     interface IPickLayer with
//         member c.Descriptor = desc
//         member c.WrittenVertexSpan = c.WrittenVertexSpan
//         member c.Clear() = c.Clear()

// type PickLayerSet() =
//     let layers = List<IPickLayer voption>()
//     member c.GetLayer<'a>(desc : PickLayerDescriptor) =
//         while layers.Count <= desc.LayerId do
//             layers.Add(ValueNone)
//         match layers.[desc.LayerId] with
//         | ValueNone ->
//             let layer = PickLayer<'a>(desc)
//             layers.[desc.LayerId] <- ValueSome (layer :> IPickLayer)
//             layer
//         | ValueSome layer -> layer :?> PickLayer<'a>
//     member c.GetValue<'a>(layerId, primitiveIndex) =
//         let layer = layers.[layerId].Value :?> PickLayer<'a>
//         layer.WrittenValueSpan.[primitiveIndex]
//     member c.TryPick(cameras : CameraSet, layerId, normPos : Vec2) =
//         if layerId >= layers.Count then ValueNone
//         else
//             match layers.[layerId] with
//             | ValueNone -> ValueNone
//             | ValueSome layer ->
//                 let span = layer.WrittenVertexSpan
//                 let viewport = cameras.[layer.Descriptor.CameraId]
//                 let worldPos = viewport.NormalizedToWorld(normPos)
//                 let primitiveResult =
//                     match layer.Descriptor.Primitive with
//                     | Triangle -> VertexPicking.TryPickTriangle(span, worldPos)
//                     | Quad -> VertexPicking.TryPickQuad(span, worldPos)
//                 match primitiveResult with
//                 | ValueNone -> ValueNone
//                 | ValueSome index ->
//                     ValueSome {
//                         LayerId = layerId
//                         PrimitiveIndex = index
//                         WorldPosition = worldPos
//                         }
//     /// Returns index of primitive containing point, if any
//     member c.TryPick(cameras : CameraSet, normPos : Vec2) =
//         let mut result = ValueNone
//         let mut i = layers.Count - 1
//         while result.IsNone && i >= 0 do
//             result <- c.TryPick(cameras, i, normPos)
//             i <- i - 1
//         result
//     member c.PickAll(param, cameras : CameraSet, layerId, normRect : Range2, action) =
//         if layerId < layers.Count then
//             match layers.[layerId] with
//             | ValueNone -> ()
//             | ValueSome layer ->
//                 let span = layer.WrittenVertexSpan
//                 let vertsPerPrimitive = SpritePrimitive.GetVertexCount(layer.Descriptor.Primitive)
//                 let viewport = cameras.[layer.Descriptor.CameraId]
//                 let worldRect = viewport.NormalizedToWorld(normRect)
//                 // Scan vertices
//                 let mut vi = 0
//                 while vi < span.Length do
//                     let v = span.[vi]
//                     if worldRect.Contains(v) then
//                         action {
//                             Param = param
//                             LayerId = layerId
//                             PrimitiveIndex = vi / vertsPerPrimitive
//                             }
//                     vi <- vi + 1
//     /// Returns index of primitive with a vertex contained within rect, if any
//     member c.PickAll(param, cameras : CameraSet, normRect : Range2, action) =
//         let mut i = layers.Count - 1
//         while i >= 0 do
//             c.PickAll(param, cameras, i, normRect, action)
//             i <- i - 1
//     member c.Flush() =
//         for i = 0 to layers.Count - 1 do
//             match layers.[i] with
//             | ValueNone -> ()
//             | ValueSome layer ->
//                 match layer.Descriptor.FlushMode with
//                 | NoFlush -> ()
//                 | FlushOnDraw -> layer.Clear()
