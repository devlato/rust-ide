// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;

#[deriving(Clone,Eq)]
pub struct FileSpan {
    beg: Option<i32>,
    end: Option<i32>,
}

impl<'self> FileSpan {
    pub fn new() -> FileSpan {
        FileSpan {
            beg: None,
            end: None,
        }
    }

    pub fn default_instance() -> &'static FileSpan {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: FileSpan = FileSpan {
//             beg: None,
//             end: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.beg {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.end {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_beg(&mut self) {
        self.beg = None;
    }

    pub fn has_beg(&self) -> bool {
        self.beg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_beg(&mut self, v: i32) {
        self.beg = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_beg(&'self mut self) -> &'self mut i32 {
        if self.beg.is_none() {
            self.beg = Some(0);
        };
        self.beg.get_mut_ref()
    }

    pub fn get_beg(&self) -> i32 {
        self.beg.unwrap_or_default(0)
    }

    pub fn clear_end(&mut self) {
        self.end = None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i32) {
        self.end = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&'self mut self) -> &'self mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        };
        self.end.get_mut_ref()
    }

    pub fn get_end(&self) -> i32 {
        self.end.unwrap_or_default(0)
    }
}

impl Message for FileSpan {
    fn new() -> FileSpan {
        FileSpan::new()
    }

    fn clear(&mut self) {
        self.clear_beg();
        self.clear_end();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.beg = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.end = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.beg.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for value in self.end.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct ProjectSpan {
    file: Option<~str>,
    span: Option<FileSpan>,
}

impl<'self> ProjectSpan {
    pub fn new() -> ProjectSpan {
        ProjectSpan {
            file: None,
            span: None,
        }
    }

    pub fn default_instance() -> &'static ProjectSpan {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: ProjectSpan = ProjectSpan {
//             file: None,
//             span: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.file {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.span {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_file(&mut self) {
        self.file = None;
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ~str) {
        self.file = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file(&'self mut self) -> &'self mut ~str {
        if self.file.is_none() {
            self.file = Some(~"");
        };
        self.file.get_mut_ref()
    }

    pub fn get_file(&'self self) -> &'self str {
        match self.file {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_span(&mut self) {
        self.span = None;
    }

    pub fn has_span(&self) -> bool {
        self.span.is_some()
    }

    // Param is passed by value, moved
    pub fn set_span(&mut self, v: FileSpan) {
        self.span = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_span(&'self mut self) -> &'self mut FileSpan {
        if self.span.is_none() {
            self.span = Some(FileSpan::new());
        };
        self.span.get_mut_ref()
    }

    pub fn get_span(&'self self) -> &'self FileSpan {
        match self.span {
            Some(ref v) => v,
            None => FileSpan::default_instance(),
        }
    }
}

impl Message for ProjectSpan {
    fn new() -> ProjectSpan {
        ProjectSpan::new()
    }

    fn clear(&mut self) {
        self.clear_file();
        self.clear_span();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.file = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FileSpan::new();
                    is.merge_message(&mut tmp);
                    self.span = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.file.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.span.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Error {
    msg: Option<~str>,
    span: Option<ProjectSpan>,
}

impl<'self> Error {
    pub fn new() -> Error {
        Error {
            msg: None,
            span: None,
        }
    }

    pub fn default_instance() -> &'static Error {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Error = Error {
//             msg: None,
//             span: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.msg {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.span {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_msg(&mut self) {
        self.msg = None;
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ~str) {
        self.msg = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&'self mut self) -> &'self mut ~str {
        if self.msg.is_none() {
            self.msg = Some(~"");
        };
        self.msg.get_mut_ref()
    }

    pub fn get_msg(&'self self) -> &'self str {
        match self.msg {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_span(&mut self) {
        self.span = None;
    }

    pub fn has_span(&self) -> bool {
        self.span.is_some()
    }

    // Param is passed by value, moved
    pub fn set_span(&mut self, v: ProjectSpan) {
        self.span = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_span(&'self mut self) -> &'self mut ProjectSpan {
        if self.span.is_none() {
            self.span = Some(ProjectSpan::new());
        };
        self.span.get_mut_ref()
    }

    pub fn get_span(&'self self) -> &'self ProjectSpan {
        match self.span {
            Some(ref v) => v,
            None => ProjectSpan::default_instance(),
        }
    }
}

impl Message for Error {
    fn new() -> Error {
        Error::new()
    }

    fn clear(&mut self) {
        self.clear_msg();
        self.clear_span();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.msg = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = ProjectSpan::new();
                    is.merge_message(&mut tmp);
                    self.span = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.msg.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.span.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct File {
    name: Option<~str>,
    content: Option<~str>,
}

impl<'self> File {
    pub fn new() -> File {
        File {
            name: None,
            content: None,
        }
    }

    pub fn default_instance() -> &'static File {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: File = File {
//             name: None,
//             content: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.content {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ~str) {
        self.name = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&'self mut self) -> &'self mut ~str {
        if self.name.is_none() {
            self.name = Some(~"");
        };
        self.name.get_mut_ref()
    }

    pub fn get_name(&'self self) -> &'self str {
        match self.name {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_content(&mut self) {
        self.content = None;
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ~str) {
        self.content = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&'self mut self) -> &'self mut ~str {
        if self.content.is_none() {
            self.content = Some(~"");
        };
        self.content.get_mut_ref()
    }

    pub fn get_content(&'self self) -> &'self str {
        match self.content {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }
}

impl Message for File {
    fn new() -> File {
        File::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_content();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.name = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.content = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.content.iter() {
            my_size += rt::string_size(2, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct NodeMarker {
    pos: Option<i32>,
    open: Option<bool>,
    node_type: Option<NodeType>,
    id: Option<i32>,
}

impl<'self> NodeMarker {
    pub fn new() -> NodeMarker {
        NodeMarker {
            pos: None,
            open: None,
            node_type: None,
            id: None,
        }
    }

    pub fn default_instance() -> &'static NodeMarker {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: NodeMarker = NodeMarker {
//             pos: None,
//             open: None,
//             node_type: None,
//             id: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.pos {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.open {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
        match self.node_type {
            Some(ref v) => {
                os.write_enum(3, *v as i32);
            },
            None => {},
        };
        match self.id {
            Some(ref v) => {
                os.write_int32(4, *v);
            },
            None => {},
        };
    }

    pub fn clear_pos(&mut self) {
        self.pos = None;
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: i32) {
        self.pos = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&'self mut self) -> &'self mut i32 {
        if self.pos.is_none() {
            self.pos = Some(0);
        };
        self.pos.get_mut_ref()
    }

    pub fn get_pos(&self) -> i32 {
        self.pos.unwrap_or_default(0)
    }

    pub fn clear_open(&mut self) {
        self.open = None;
    }

    pub fn has_open(&self) -> bool {
        self.open.is_some()
    }

    // Param is passed by value, moved
    pub fn set_open(&mut self, v: bool) {
        self.open = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_open(&'self mut self) -> &'self mut bool {
        if self.open.is_none() {
            self.open = Some(false);
        };
        self.open.get_mut_ref()
    }

    pub fn get_open(&self) -> bool {
        self.open.unwrap_or_default(false)
    }

    pub fn clear_node_type(&mut self) {
        self.node_type = None;
    }

    pub fn has_node_type(&self) -> bool {
        self.node_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_type(&mut self, v: NodeType) {
        self.node_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_type(&'self mut self) -> &'self mut NodeType {
        if self.node_type.is_none() {
            self.node_type = Some(NodeType::new(0));
        };
        self.node_type.get_mut_ref()
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type.unwrap_or_default(NodeType::new(0))
    }

    pub fn clear_id(&mut self) {
        self.id = None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'self mut self) -> &'self mut i32 {
        if self.id.is_none() {
            self.id = Some(0);
        };
        self.id.get_mut_ref()
    }

    pub fn get_id(&self) -> i32 {
        self.id.unwrap_or_default(0)
    }
}

impl Message for NodeMarker {
    fn new() -> NodeMarker {
        NodeMarker::new()
    }

    fn clear(&mut self) {
        self.clear_pos();
        self.clear_open();
        self.clear_node_type();
        self.clear_id();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.pos = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.open = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = NodeType::new(is.read_int32());
                    self.node_type = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.id = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.pos.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        if self.open.is_some() {
            my_size += 2;
        };
        for value in self.node_type.iter() {
            my_size += rt::enum_size(3, *value);
        };
        for value in self.id.iter() {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct LazyAst {
    markers: ~[NodeMarker],
}

impl<'self> LazyAst {
    pub fn new() -> LazyAst {
        LazyAst {
            markers: ~[],
        }
    }

    pub fn default_instance() -> &'static LazyAst {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: LazyAst = LazyAst {
//             markers: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.markers.iter() {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_markers(&mut self) {
        self.markers.clear();
    }

    // Param is passed by value, moved
    pub fn set_markers(&mut self, v: ~[NodeMarker]) {
        self.markers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_markers(&'self mut self) -> &'self mut ~[NodeMarker] {
        &mut self.markers
    }

    pub fn get_markers(&'self self) -> &'self [NodeMarker] {
        rt::as_slice_tmp(&self.markers)
    }

    pub fn add_markers(&mut self, v: NodeMarker) {
        self.markers.push(v);
    }
}

impl Message for LazyAst {
    fn new() -> LazyAst {
        LazyAst::new()
    }

    fn clear(&mut self) {
        self.clear_markers();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                5 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = NodeMarker::new();
                    is.merge_message(&mut tmp);
                    self.markers.push(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.markers.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct JumpMap {
    entries: ~[JumpMap_Entry],
}

impl<'self> JumpMap {
    pub fn new() -> JumpMap {
        JumpMap {
            entries: ~[],
        }
    }

    pub fn default_instance() -> &'static JumpMap {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: JumpMap = JumpMap {
//             entries: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.entries.iter() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ~[JumpMap_Entry]) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&'self mut self) -> &'self mut ~[JumpMap_Entry] {
        &mut self.entries
    }

    pub fn get_entries(&'self self) -> &'self [JumpMap_Entry] {
        rt::as_slice_tmp(&self.entries)
    }

    pub fn add_entries(&mut self, v: JumpMap_Entry) {
        self.entries.push(v);
    }
}

impl Message for JumpMap {
    fn new() -> JumpMap {
        JumpMap::new()
    }

    fn clear(&mut self) {
        self.clear_entries();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = JumpMap_Entry::new();
                    is.merge_message(&mut tmp);
                    self.entries.push(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.entries.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct JumpMap_Entry {
    source_node_id: Option<i32>,
    target_node_id: Option<i32>,
}

impl<'self> JumpMap_Entry {
    pub fn new() -> JumpMap_Entry {
        JumpMap_Entry {
            source_node_id: None,
            target_node_id: None,
        }
    }

    pub fn default_instance() -> &'static JumpMap_Entry {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: JumpMap_Entry = JumpMap_Entry {
//             source_node_id: None,
//             target_node_id: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.source_node_id {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.target_node_id {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_source_node_id(&mut self) {
        self.source_node_id = None;
    }

    pub fn has_source_node_id(&self) -> bool {
        self.source_node_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_node_id(&mut self, v: i32) {
        self.source_node_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_node_id(&'self mut self) -> &'self mut i32 {
        if self.source_node_id.is_none() {
            self.source_node_id = Some(0);
        };
        self.source_node_id.get_mut_ref()
    }

    pub fn get_source_node_id(&self) -> i32 {
        self.source_node_id.unwrap_or_default(0)
    }

    pub fn clear_target_node_id(&mut self) {
        self.target_node_id = None;
    }

    pub fn has_target_node_id(&self) -> bool {
        self.target_node_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_node_id(&mut self, v: i32) {
        self.target_node_id = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_node_id(&'self mut self) -> &'self mut i32 {
        if self.target_node_id.is_none() {
            self.target_node_id = Some(0);
        };
        self.target_node_id.get_mut_ref()
    }

    pub fn get_target_node_id(&self) -> i32 {
        self.target_node_id.unwrap_or_default(0)
    }
}

impl Message for JumpMap_Entry {
    fn new() -> JumpMap_Entry {
        JumpMap_Entry::new()
    }

    fn clear(&mut self) {
        self.clear_source_node_id();
        self.clear_target_node_id();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.source_node_id = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.target_node_id = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.source_node_id.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for value in self.target_node_id.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Request {
    ping: Option<Request_Ping>,
    analyze: Option<Request_Analyze>,
    unknown_command: Option<Request_Unknown>,
}

impl<'self> Request {
    pub fn new() -> Request {
        Request {
            ping: None,
            analyze: None,
            unknown_command: None,
        }
    }

    pub fn default_instance() -> &'static Request {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Request = Request {
//             ping: None,
//             analyze: None,
//             unknown_command: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.ping {
            Some(ref v) => {
                os.write_tag(1, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.analyze {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.unknown_command {
            Some(ref v) => {
                os.write_tag(999, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_ping(&mut self) {
        self.ping = None;
    }

    pub fn has_ping(&self) -> bool {
        self.ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: Request_Ping) {
        self.ping = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping(&'self mut self) -> &'self mut Request_Ping {
        if self.ping.is_none() {
            self.ping = Some(Request_Ping::new());
        };
        self.ping.get_mut_ref()
    }

    pub fn get_ping(&'self self) -> &'self Request_Ping {
        match self.ping {
            Some(ref v) => v,
            None => Request_Ping::default_instance(),
        }
    }

    pub fn clear_analyze(&mut self) {
        self.analyze = None;
    }

    pub fn has_analyze(&self) -> bool {
        self.analyze.is_some()
    }

    // Param is passed by value, moved
    pub fn set_analyze(&mut self, v: Request_Analyze) {
        self.analyze = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_analyze(&'self mut self) -> &'self mut Request_Analyze {
        if self.analyze.is_none() {
            self.analyze = Some(Request_Analyze::new());
        };
        self.analyze.get_mut_ref()
    }

    pub fn get_analyze(&'self self) -> &'self Request_Analyze {
        match self.analyze {
            Some(ref v) => v,
            None => Request_Analyze::default_instance(),
        }
    }

    pub fn clear_unknown_command(&mut self) {
        self.unknown_command = None;
    }

    pub fn has_unknown_command(&self) -> bool {
        self.unknown_command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unknown_command(&mut self, v: Request_Unknown) {
        self.unknown_command = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unknown_command(&'self mut self) -> &'self mut Request_Unknown {
        if self.unknown_command.is_none() {
            self.unknown_command = Some(Request_Unknown::new());
        };
        self.unknown_command.get_mut_ref()
    }

    pub fn get_unknown_command(&'self self) -> &'self Request_Unknown {
        match self.unknown_command {
            Some(ref v) => v,
            None => Request_Unknown::default_instance(),
        }
    }
}

impl Message for Request {
    fn new() -> Request {
        Request::new()
    }

    fn clear(&mut self) {
        self.clear_ping();
        self.clear_analyze();
        self.clear_unknown_command();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Request_Ping::new();
                    is.merge_message(&mut tmp);
                    self.ping = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Request_Analyze::new();
                    is.merge_message(&mut tmp);
                    self.analyze = Some(tmp);
                },
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Request_Unknown::new();
                    is.merge_message(&mut tmp);
                    self.unknown_command = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.ping.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.analyze.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.unknown_command.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Request_Ping {
    dummy: bool,
}

impl<'self> Request_Ping {
    pub fn new() -> Request_Ping {
        Request_Ping {
            dummy: false,
        }
    }

    pub fn default_instance() -> &'static Request_Ping {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Request_Ping = Request_Ping {
//             dummy: false,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
    }
}

impl Message for Request_Ping {
    fn new() -> Request_Ping {
        Request_Ping::new()
    }

    fn clear(&mut self) {
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Request_Analyze {
    file: Option<File>,
}

impl<'self> Request_Analyze {
    pub fn new() -> Request_Analyze {
        Request_Analyze {
            file: None,
        }
    }

    pub fn default_instance() -> &'static Request_Analyze {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Request_Analyze = Request_Analyze {
//             file: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.file {
            Some(ref v) => {
                os.write_tag(1, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_file(&mut self) {
        self.file = None;
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: File) {
        self.file = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file(&'self mut self) -> &'self mut File {
        if self.file.is_none() {
            self.file = Some(File::new());
        };
        self.file.get_mut_ref()
    }

    pub fn get_file(&'self self) -> &'self File {
        match self.file {
            Some(ref v) => v,
            None => File::default_instance(),
        }
    }
}

impl Message for Request_Analyze {
    fn new() -> Request_Analyze {
        Request_Analyze::new()
    }

    fn clear(&mut self) {
        self.clear_file();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = File::new();
                    is.merge_message(&mut tmp);
                    self.file = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.file.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Request_Unknown {
    dummy: bool,
}

impl<'self> Request_Unknown {
    pub fn new() -> Request_Unknown {
        Request_Unknown {
            dummy: false,
        }
    }

    pub fn default_instance() -> &'static Request_Unknown {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Request_Unknown = Request_Unknown {
//             dummy: false,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
    }
}

impl Message for Request_Unknown {
    fn new() -> Request_Unknown {
        Request_Unknown::new()
    }

    fn clear(&mut self) {
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Response {
    ping: Option<Response_Ping>,
    analyze: Option<Response_Analyze>,
}

impl<'self> Response {
    pub fn new() -> Response {
        Response {
            ping: None,
            analyze: None,
        }
    }

    pub fn default_instance() -> &'static Response {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Response = Response {
//             ping: None,
//             analyze: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.ping {
            Some(ref v) => {
                os.write_tag(1, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.analyze {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_ping(&mut self) {
        self.ping = None;
    }

    pub fn has_ping(&self) -> bool {
        self.ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: Response_Ping) {
        self.ping = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping(&'self mut self) -> &'self mut Response_Ping {
        if self.ping.is_none() {
            self.ping = Some(Response_Ping::new());
        };
        self.ping.get_mut_ref()
    }

    pub fn get_ping(&'self self) -> &'self Response_Ping {
        match self.ping {
            Some(ref v) => v,
            None => Response_Ping::default_instance(),
        }
    }

    pub fn clear_analyze(&mut self) {
        self.analyze = None;
    }

    pub fn has_analyze(&self) -> bool {
        self.analyze.is_some()
    }

    // Param is passed by value, moved
    pub fn set_analyze(&mut self, v: Response_Analyze) {
        self.analyze = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_analyze(&'self mut self) -> &'self mut Response_Analyze {
        if self.analyze.is_none() {
            self.analyze = Some(Response_Analyze::new());
        };
        self.analyze.get_mut_ref()
    }

    pub fn get_analyze(&'self self) -> &'self Response_Analyze {
        match self.analyze {
            Some(ref v) => v,
            None => Response_Analyze::default_instance(),
        }
    }
}

impl Message for Response {
    fn new() -> Response {
        Response::new()
    }

    fn clear(&mut self) {
        self.clear_ping();
        self.clear_analyze();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Response_Ping::new();
                    is.merge_message(&mut tmp);
                    self.ping = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Response_Analyze::new();
                    is.merge_message(&mut tmp);
                    self.analyze = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.ping.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.analyze.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Response_Ping {
    dummy: bool,
}

impl<'self> Response_Ping {
    pub fn new() -> Response_Ping {
        Response_Ping {
            dummy: false,
        }
    }

    pub fn default_instance() -> &'static Response_Ping {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Response_Ping = Response_Ping {
//             dummy: false,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
    }
}

impl Message for Response_Ping {
    fn new() -> Response_Ping {
        Response_Ping::new()
    }

    fn clear(&mut self) {
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub struct Response_Analyze {
    ast: Option<LazyAst>,
    jump_map: Option<JumpMap>,
    errors: ~[Error],
}

impl<'self> Response_Analyze {
    pub fn new() -> Response_Analyze {
        Response_Analyze {
            ast: None,
            jump_map: None,
            errors: ~[],
        }
    }

    pub fn default_instance() -> &'static Response_Analyze {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: Response_Analyze = Response_Analyze {
//             ast: None,
//             jump_map: None,
//             errors: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.ast {
            Some(ref v) => {
                os.write_tag(1, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.jump_map {
            Some(ref v) => {
                os.write_tag(2, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        for v in self.errors.iter() {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_ast(&mut self) {
        self.ast = None;
    }

    pub fn has_ast(&self) -> bool {
        self.ast.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ast(&mut self, v: LazyAst) {
        self.ast = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ast(&'self mut self) -> &'self mut LazyAst {
        if self.ast.is_none() {
            self.ast = Some(LazyAst::new());
        };
        self.ast.get_mut_ref()
    }

    pub fn get_ast(&'self self) -> &'self LazyAst {
        match self.ast {
            Some(ref v) => v,
            None => LazyAst::default_instance(),
        }
    }

    pub fn clear_jump_map(&mut self) {
        self.jump_map = None;
    }

    pub fn has_jump_map(&self) -> bool {
        self.jump_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jump_map(&mut self, v: JumpMap) {
        self.jump_map = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jump_map(&'self mut self) -> &'self mut JumpMap {
        if self.jump_map.is_none() {
            self.jump_map = Some(JumpMap::new());
        };
        self.jump_map.get_mut_ref()
    }

    pub fn get_jump_map(&'self self) -> &'self JumpMap {
        match self.jump_map {
            Some(ref v) => v,
            None => JumpMap::default_instance(),
        }
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ~[Error]) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&'self mut self) -> &'self mut ~[Error] {
        &mut self.errors
    }

    pub fn get_errors(&'self self) -> &'self [Error] {
        rt::as_slice_tmp(&self.errors)
    }

    pub fn add_errors(&mut self, v: Error) {
        self.errors.push(v);
    }
}

impl Message for Response_Analyze {
    fn new() -> Response_Analyze {
        Response_Analyze::new()
    }

    fn clear(&mut self) {
        self.clear_ast();
        self.clear_jump_map();
        self.clear_errors();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = LazyAst::new();
                    is.merge_message(&mut tmp);
                    self.ast = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = JumpMap::new();
                    is.merge_message(&mut tmp);
                    self.jump_map = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Error::new();
                    is.merge_message(&mut tmp);
                    self.errors.push(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.ast.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.jump_map.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.errors.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

#[deriving(Clone,Eq)]
pub enum NodeType {
    NodeRoot = 1,
    NodeFn = 2,
    NodeStmt = 3,
    NodeStructDef = 4,
    NodeTy = 5,
    NodeOther = 100,
}

impl NodeType {
    pub fn new(value: i32) -> NodeType {
        match value {
            1 => NodeRoot,
            2 => NodeFn,
            3 => NodeStmt,
            4 => NodeStructDef,
            5 => NodeTy,
            100 => NodeOther,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }
}
