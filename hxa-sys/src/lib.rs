/*!
HxA 3D asset format
===================

HxA is a interchangeable graphics asset format. Written by Eskil Steenberg. @quelsolaar / eskil 'at' obsession 'dot' se / www.quelsolaar.com

Rational:
---------

- Does the world need another Graphics file format?

Unfortunately, Yes. All existing formats are either too large and complicated to be implemented from scratch, or don't have some basic features needed in modern computer graphics.

- Who is this format for?

For people who want a capable open Graphics format that can be implemented from scratch in a few hours. It is ideal for graphics researchers, game developers or other people who wants to build custom graphics pipelines. Given how easy it is to parse and write, it should be easy to write utilities that process assets to preform tasks like: generating normals, lightmaps, tangent spaces, Error detection, GPU optimization, LOD generation, and UV mapping.

- Why store images in the format when there are so many good image formats already?

Yes there are, but only for 2D RGB/RGBA images. A lot of computer graphics rendering rely on 1D, 3D, CUBE, Multilayer, multi channel, floating point bitmap buffers. There almost no formats for this kind of data. Also 3D files that reference separate image files rely on file paths, and this often creates issues when the assets are moved. By including the texture data in the files directly the assets become self contained.

- Why doesn't the format support <insert whatever>?

Because the entire point is to make a format that can be implemented. Features like NURBSs, Construction history, or BSP trees would make the format too large to serve its purpose. The facilities of the formats to store meta data should make the format flexible enough for most uses. Adding HxA support should be something anyone can do in a days work.

Structure:
----------

HxA is designed to be extremely simple to parse, and is therefor based around conventions. It has a few basic structures, and depending on how they are used they mean different things. This means that you can implement a tool that loads the entire file, modifies the parts it cares about and leaves the rest intact. It is also possible to write a tool that makes all data in the file editable without the need to understand its use. It is also possible for anyone to use the format to store data axillary data. Anyone who wants to store data not covered by a convention can submit a convention to extend the format. There should never be a convention for storing the same data in two differed ways.

The data is story in a number of nodes that are stored in an array. Each node stores an array of meta data. Meta data can describe anything you want, and a lot of conventions will use meta data to store additional information, for things like transforms, lights, shaders and animation.
Data for Vertices, Corners, Faces, and Pixels are stored in named layer stacks. Each stack consists of a number of named layers. All layers in the stack have the same number of elements. Each layer describes one property of the primitive. Each layer can have multiple channels and each layer can store data of a different type.

HaX stores 3 kinds of nodes
- Pixel data.
- Polygon geometry data.
- Meta data only.

Pixel Nodes stores pixels in a layer stack. A layer may store things like Albedo, Roughness, Reflectance, Light maps, Masks, Normal maps, and Displacement. Layers use the channels of the layers to store things like color. The length of the layer stack is determined by the type and dimensions stored in the

Geometry data is stored in 3 separate layer stacks for: vertex data, corner data and face data. The vertex data stores things like veritices, blend shapes, weight maps, and vertex colors. The first layer in a vertex stack has to be a 3 channel layer named "position" describing the base position of the vertices. The corner stack describes data per corner or edge of the polygons. It can be used for things like UV, normals, and adjacency. The first layer in a corner stack has to be a 1 channel integer layer named "index" describing the vertices used to form polygons. The last value in each polygon has a negative - 1 index to indicate the end of the polygon.

Example:

A quad and a tri with the vertex index: `[0, 1, 2, 3] [1, 4, 2]`

is stored: `[0, 1, 2, -4, 1, 4, -3]`

The face stack stores values per face. the length of the face stack has to match the number of negative values in the index layer in the corner stack. The face stack can be used to store things like material index.

Storage
-------

All data is stored in little endian byte order with no padding. The layout mirrors the structs defined below with a few exceptions.

All names are stored as a 8bit unsigned integer indicating the length of the name followed by that many characters. Termination is not stored in the file.

Text strings stored in meta data are stored the same way as names, but instead of a 8bit unsigned integer a 32bit unsigned integer is used.
*/

pub const HXA_VERSION_API: &[u8; 4usize] = b"0.3\0";
pub const HXA_VERSION_FORMAT: u32 = 3;
pub const HXA_NAME_MAX_LENGTH: u32 = 256;

pub const HXA_CONVENTION_HARD_BASE_VERTEX_LAYER_NAME: &[u8; 7usize] = b"vertex\0";
pub const HXA_CONVENTION_HARD_BASE_VERTEX_LAYER_ID: u32 = 0;
pub const HXA_CONVENTION_HARD_BASE_VERTEX_LAYER_COMPONENTS: u32 = 3;
pub const HXA_CONVENTION_HARD_BASE_CORNER_LAYER_NAME: &[u8; 10usize] = b"reference\0";
pub const HXA_CONVENTION_HARD_BASE_CORNER_LAYER_ID: u32 = 0;
pub const HXA_CONVENTION_HARD_BASE_CORNER_LAYER_COMPONENTS: u32 = 1;
pub const HXA_CONVENTION_HARD_EDGE_NEIGHBOUR_LAYER_NAME: &[u8; 10usize] = b"neighbour\0";

pub const HXA_CONVENTION_SOFT_LAYER_SEQUENCE0: &[u8; 9usize] = b"sequence\0";
pub const HXA_CONVENTION_SOFT_LAYER_UV0: &[u8; 3usize] = b"uv\0";
pub const HXA_CONVENTION_SOFT_LAYER_NORMALS: &[u8; 7usize] = b"normal\0";
pub const HXA_CONVENTION_SOFT_LAYER_BINORMAL: &[u8; 9usize] = b"binormal\0";
pub const HXA_CONVENTION_SOFT_LAYER_TANGENT: &[u8; 8usize] = b"tangent\0";
pub const HXA_CONVENTION_SOFT_LAYER_COLOR: &[u8; 6usize] = b"color\0";
pub const HXA_CONVENTION_SOFT_LAYER_CREASES: &[u8; 8usize] = b"creases\0";
pub const HXA_CONVENTION_SOFT_LAYER_SELECTION: &[u8; 7usize] = b"select\0";
pub const HXA_CONVENTION_SOFT_LAYER_SKIN_WEIGHT: &[u8; 15usize] = b"skining_weight\0";
pub const HXA_CONVENTION_SOFT_LAYER_SKIN_REFERENCE: &[u8; 18usize] = b"skining_reference\0";
pub const HXA_CONVENTION_SOFT_LAYER_BLENDSHAPE: &[u8; 11usize] = b"blendshape\0";
pub const HXA_CONVENTION_SOFT_LAYER_ADD_BLENDSHAPE: &[u8; 14usize] = b"addblendshape\0";
pub const HXA_CONVENTION_SOFT_LAYER_MATERIAL_ID: &[u8; 9usize] = b"material\0";
pub const HXA_CONVENTION_SOFT_LAYER_GROUP_ID: &[u8; 6usize] = b"group\0";
pub const HXA_CONVENTION_SOFT_ALBEDO: &[u8; 7usize] = b"albedo\0";
pub const HXA_CONVENTION_SOFT_LIGHT: &[u8; 6usize] = b"light\0";
pub const HXA_CONVENTION_SOFT_DISPLACEMENT: &[u8; 13usize] = b"displacement\0";
pub const HXA_CONVENTION_SOFT_DISTORTION: &[u8; 11usize] = b"distortion\0";
pub const HXA_CONVENTION_SOFT_AMBIENT_OCCLUSION: &[u8; 18usize] = b"ambient_occlusion\0";
pub const HXA_CONVENTION_SOFT_NAME: &[u8; 5usize] = b"name\0";
pub const HXA_CONVENTION_SOFT_TRANSFORM: &[u8; 10usize] = b"transform\0";

#[allow(non_camel_case_types)]
pub type hxa_uint8 = ::std::os::raw::c_uchar;
#[allow(non_camel_case_types)]
pub type hxa_int32 = ::std::os::raw::c_int;
#[allow(non_camel_case_types)]
pub type hxa_uint32 = ::std::os::raw::c_uint;
#[allow(non_camel_case_types)]
pub type hxa_int64 = ::std::os::raw::c_longlong;
#[allow(non_camel_case_types)]
pub type hxa_uint64 = ::std::os::raw::c_ulonglong;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HXANodeType {
    /// node only containing meta data.
    HXA_NT_META_ONLY = 0,
    /// node containing a geometry mesh, and meta data.
    HXA_NT_GEOMETRY = 1,
    /// node containing a 1D, 2D, 3D, or Cube image, and meta data.
    HXA_NT_IMAGE = 2,
    /// the number of different nodes that can be stored in the file.
    HXA_NT_COUNT = 3,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HXAImageType {
    /// 6 sided qube, in the order of: +x, -x, +y, -y, +z, -z.
    HXA_IT_CUBE_IMAGE = 0,
    /// One dimentional pixel data.
    HXA_IT_1D_IMAGE = 1,
    /// Two dimentional pixel data.
    HXA_IT_2D_IMAGE = 2,
    /// Three dimentional pixel data.
    HXA_IT_3D_IMAGE = 3,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HXAMetaDataType {
    HXA_MDT_INT64 = 0,
    HXA_MDT_DOUBLE = 1,
    HXA_MDT_NODE = 2,
    HXA_MDT_TEXT = 3,
    HXA_MDT_BINARY = 4,
    HXA_MDT_META = 5,
    HXA_MDT_COUNT = 6,
}

/// meta data key/value store
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HXAMeta {
    /// name of the meta data value.
    pub name: [::std::os::raw::c_char; 256usize],
    /// type of value. Stored in the file as a uint8.
    pub type_: HXAMetaDataType,
    /// how many values are stored / the length of the stored text string (excluding termination)
    pub array_length: hxa_uint32,
    pub value: HXAMetaValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HXAMetaValue {
    /// integer values
    pub int64_value: *mut hxa_uint64,
    /// double values
    pub double_value: *mut f64,
    /// a reference to another node
    pub node_value: *mut hxa_uint32,
    /// text string
    pub text_value: *mut ::std::os::raw::c_char,
    /// binary data string
    pub bin_value: *mut ::std::os::raw::c_uchar,
    /// Meta structures
    pub array_of_meta: *mut ::std::os::raw::c_void,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum HXALayerDataType {
    /// 8bit unsigned integer
    HXA_LDT_UINT8 = 0,
    /// 32bit signed integer
    HXA_LDT_INT32 = 1,
    /// 32bit IEEE 754 floating point value
    HXA_LDT_FLOAT = 2,
    /// 64bit IEEE 754 floating point value
    HXA_LDT_DOUBLE = 3,
    /// number of types supported by layers
    HXA_LDT_COUNT = 4,
}

/// Layers are arrays of data used to store geometry and pixel data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HXALayer {
    /// name of the layer. List of predefined names for common usages like uv, reference, blendshapes, weights ...
    pub name: [::std::os::raw::c_char; 256usize],
    /// 2 for uv, 3 for xyz or rgb, 4 for rgba. from 1 - 255 is legal.
    pub components: hxa_uint8,
    /// Stored in the file as a uint8.
    pub type_: HXALayerDataType,
    pub data: HXALayerData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HXALayerData {
    pub uint8_data: *mut hxa_uint8,
    pub int32_data: *mut hxa_int32,
    pub float_data: *mut f32,
    pub double_data: *mut f64,
}

/**
Layers stacks are arrays of layers where all the layers have the same number of entries (polygons, edges, vertices or pixels)

```ignore
// C Code
vertex_layer_stack.layers[0].name = HXA_CONVENTION_HARD_BASE_VERTEX_LAYER_NAME;
vertex_layer_stack.layers[0].data.float_data = {0, 0, 0, 1, 1, 1};
vertex_layer_stack.layers[0].components = 3;

corner_layer_stack.layers[0].name = HXA_CONVENTION_HARD_BASE_REFERECNE_LAYER_NAME;
corner_layer_stack.layers[0].data.int32_data = {0, 1, -3};
corner_layer_stack.layers[0].components = 1;
corner_layer_stack.layers[1].name = "UV";
corner_layer_stack.layers[1].data.double_data = {1, 1, 0, 0, 1, 0};
corner_layer_stack.layers[1].components = 2;
```

```ignore
// C Code
HXANode *node;
node = hxa_util_load("file.hxa", TRUE);
hxa_util_triangulate_node(node, 3);
node....
```
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HXALayerStack {
    /// the number of loayers in a stack.
    pub layer_count: hxa_uint32,
    /// An array of layers.
    pub layers: *mut HXALayer,
}

/// A file consists of an array of nodes, All noides have meta data. Geometry nodes have geometry, image nodes have pixels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HXANode {
    /// what type of node is this? Stored in the file as a uint8.
    pub type_: HXANodeType,
    /// how many meta data key/values are stored in the node
    pub meta_data_count: hxa_uint32,
    /// array of key/values
    pub meta_data: *mut HXAMeta,
    pub content: HXANodeContent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HXANodeContent {
    pub geometry: HXANodeContentGeometry,
    pub image: HXANodeContentImage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HXANodeContentGeometry {
    /// number of vertices
    pub vertex_count: hxa_uint32,
    /// stack of vertex arrays. the first layer is always the vertex positions
    pub vertex_stack: HXALayerStack,
    /// number of corners
    pub edge_corner_count: hxa_uint32,
    /// stack of corner arrays, the first layer is always a reference array (see below)
    pub corner_stack: HXALayerStack,
    /// stack of edge arrays
    pub edge_stack: HXALayerStack,
    /// number of polygons
    pub face_count: hxa_uint32,
    /// stack of per polygon data.
    pub face_stack: HXALayerStack,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HXANodeContentImage {
    /// type of image
    pub type_: HXAImageType,
    /// resolution i X, Y and Z dimension;
    pub resolution: [hxa_uint32; 3usize],
    /// the number of values in the stack is equal to the number of pixels depending on resolution
    pub image_stack: HXALayerStack,
}

/// The file begins with a file identifier. it always has to be the 4 bytes "HxA". Since the magic number is always the same we don't store it in this structure even if it is always present in files.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HXAFile {
    /// HXA_VERSION_FORMAT
    pub version: hxa_uint8,
    /// number of nodes in the file
    pub node_count: hxa_uint32,
    /// array of nodes.
    pub node_array: *mut HXANode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXAMetaValue() {
        assert_eq!(
            ::std::mem::size_of::<HXAMetaValue>(),
            8usize,
            concat!("Size of: ", stringify!(HXAMetaValue))
        );
        assert_eq!(
            ::std::mem::align_of::<HXAMetaValue>(),
            8usize,
            concat!("Alignment of ", stringify!(HXAMetaValue))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).int64_value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(int64_value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).double_value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(double_value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).node_value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(node_value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).text_value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(text_value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).bin_value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(bin_value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMetaValue>())).array_of_meta as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMetaValue),
                "::",
                stringify!(array_of_meta)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXAMeta() {
        assert_eq!(
            ::std::mem::size_of::<HXAMeta>(),
            272usize,
            concat!("Size of: ", stringify!(HXAMeta))
        );
        assert_eq!(
            ::std::mem::align_of::<HXAMeta>(),
            8usize,
            concat!("Alignment of ", stringify!(HXAMeta))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMeta>())).name as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMeta),
                "::",
                stringify!(name)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMeta>())).type_ as *const _ as usize },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMeta),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMeta>())).array_length as *const _ as usize },
            260usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMeta),
                "::",
                stringify!(array_length)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAMeta>())).value as *const _ as usize },
            264usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAMeta),
                "::",
                stringify!(value)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXALayerData() {
        assert_eq!(
            ::std::mem::size_of::<HXALayerData>(),
            8usize,
            concat!("Size of: ", stringify!(HXALayerData))
        );
        assert_eq!(
            ::std::mem::align_of::<HXALayerData>(),
            8usize,
            concat!("Alignment of ", stringify!(HXALayerData))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerData>())).uint8_data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerData),
                "::",
                stringify!(uint8_data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerData>())).int32_data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerData),
                "::",
                stringify!(int32_data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerData>())).float_data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerData),
                "::",
                stringify!(float_data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerData>())).double_data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerData),
                "::",
                stringify!(double_data)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXALayer() {
        assert_eq!(
            ::std::mem::size_of::<HXALayer>(),
            272usize,
            concat!("Size of: ", stringify!(HXALayer))
        );
        assert_eq!(
            ::std::mem::align_of::<HXALayer>(),
            8usize,
            concat!("Alignment of ", stringify!(HXALayer))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayer>())).name as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayer),
                "::",
                stringify!(name)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayer>())).components as *const _ as usize },
            256usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayer),
                "::",
                stringify!(components)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayer>())).type_ as *const _ as usize },
            260usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayer),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayer>())).data as *const _ as usize },
            264usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayer),
                "::",
                stringify!(data)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXALayerStack() {
        assert_eq!(
            ::std::mem::size_of::<HXALayerStack>(),
            16usize,
            concat!("Size of: ", stringify!(HXALayerStack))
        );
        assert_eq!(
            ::std::mem::align_of::<HXALayerStack>(),
            8usize,
            concat!("Alignment of ", stringify!(HXALayerStack))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerStack>())).layer_count as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerStack),
                "::",
                stringify!(layer_count)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXALayerStack>())).layers as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(HXALayerStack),
                "::",
                stringify!(layers)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXANodeContentGeometry() {
        assert_eq!(
            ::std::mem::size_of::<HXANodeContentGeometry>(),
            88usize,
            concat!("Size of: ", stringify!(HXANodeContentGeometry))
        );
        assert_eq!(
            ::std::mem::align_of::<HXANodeContentGeometry>(),
            8usize,
            concat!("Alignment of ", stringify!(HXANodeContentGeometry))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).vertex_count as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(vertex_count)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).vertex_stack as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(vertex_stack)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).edge_corner_count as *const _
                    as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(edge_corner_count)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).corner_stack as *const _ as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(corner_stack)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).edge_stack as *const _ as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(edge_stack)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).face_count as *const _ as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(face_count)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentGeometry>())).face_stack as *const _ as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentGeometry),
                "::",
                stringify!(face_stack)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXANodeContentImage() {
        assert_eq!(
            ::std::mem::size_of::<HXANodeContentImage>(),
            32usize,
            concat!("Size of: ", stringify!(HXANodeContentImage))
        );
        assert_eq!(
            ::std::mem::align_of::<HXANodeContentImage>(),
            8usize,
            concat!("Alignment of ", stringify!(HXANodeContentImage))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANodeContentImage>())).type_ as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentImage),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentImage>())).resolution as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentImage),
                "::",
                stringify!(resolution)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<HXANodeContentImage>())).image_stack as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContentImage),
                "::",
                stringify!(image_stack)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXANodeContent() {
        assert_eq!(
            ::std::mem::size_of::<HXANodeContent>(),
            88usize,
            concat!("Size of: ", stringify!(HXANodeContent))
        );
        assert_eq!(
            ::std::mem::align_of::<HXANodeContent>(),
            8usize,
            concat!("Alignment of ", stringify!(HXANodeContent))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANodeContent>())).geometry as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContent),
                "::",
                stringify!(geometry)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANodeContent>())).image as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANodeContent),
                "::",
                stringify!(image)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXANode() {
        assert_eq!(
            ::std::mem::size_of::<HXANode>(),
            104usize,
            concat!("Size of: ", stringify!(HXANode))
        );
        assert_eq!(
            ::std::mem::align_of::<HXANode>(),
            8usize,
            concat!("Alignment of ", stringify!(HXANode))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANode>())).type_ as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANode),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANode>())).meta_data_count as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANode),
                "::",
                stringify!(meta_data_count)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANode>())).meta_data as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANode),
                "::",
                stringify!(meta_data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXANode>())).content as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(HXANode),
                "::",
                stringify!(content)
            )
        );
    }

    #[test]
    #[allow(deref_nullptr)]
    #[allow(non_snake_case)]
    fn bindgen_test_layout_HXAFile() {
        assert_eq!(
            ::std::mem::size_of::<HXAFile>(),
            16usize,
            concat!("Size of: ", stringify!(HXAFile))
        );
        assert_eq!(
            ::std::mem::align_of::<HXAFile>(),
            8usize,
            concat!("Alignment of ", stringify!(HXAFile))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAFile>())).version as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAFile),
                "::",
                stringify!(version)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAFile>())).node_count as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAFile),
                "::",
                stringify!(node_count)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<HXAFile>())).node_array as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(HXAFile),
                "::",
                stringify!(node_array)
            )
        );
    }
}
