// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;

#[deriving(Clone,Eq)]
pub struct Request {
    ping: Option<Request_Ping>,
    unknown_command: Option<Request_Unknown>,
}

impl<'self> Request {
    pub fn new() -> Request {
        Request {
            ping: None,
            unknown_command: None,
        }
    }

    pub fn default_instance() -> &'static Request {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Request = Request {
//             ping: None,
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
}

impl<'self> Response {
    pub fn new() -> Response {
        Response {
            ping: None,
        }
    }

    pub fn default_instance() -> &'static Response {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Response = Response {
//             ping: None,
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
}

impl Message for Response {
    fn new() -> Response {
        Response::new()
    }

    fn clear(&mut self) {
        self.clear_ping();
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
