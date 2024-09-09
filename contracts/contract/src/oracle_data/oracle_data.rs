// Generated by Molecule 0.8.0

use molecule :: prelude :: * ;
use crate::common::common::*;
# [derive (Clone)] pub struct PriceStruct (molecule :: bytes :: Bytes) ; impl :: core :: fmt :: LowerHex for PriceStruct { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl :: core :: fmt :: Debug for PriceStruct { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl :: core :: fmt :: Display for PriceStruct { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} {{ " , Self :: NAME) ? ; write ! (f , "{}: {}" , "udt_symbol" , self . udt_symbol ()) ? ; write ! (f , ", {}: {}" , "price" , self . price ()) ? ; write ! (f , ", {}: {}" , "decimal" , self . decimal ()) ? ; write ! (f , ", {}: {}" , "timestamp" , self . timestamp ()) ? ; write ! (f , ", {}: {}" , "udt_type_hash" , self . udt_type_hash ()) ? ; let extra_count = self . count_extra_fields () ; if extra_count != 0 { write ! (f , ", .. ({} fields)" , extra_count) ? ; } write ! (f , " }}") } } impl :: core :: default :: Default for PriceStruct { fn default () -> Self { let v = molecule :: bytes :: Bytes :: from_static (& Self :: DEFAULT_VALUE) ; PriceStruct :: new_unchecked (v) } } impl PriceStruct { const DEFAULT_VALUE : [u8 ; 73] = [73 , 0 , 0 , 0 , 24 , 0 , 0 , 0 , 28 , 0 , 0 , 0 , 60 , 0 , 0 , 0 , 61 , 0 , 0 , 0 , 69 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,] ; pub const FIELD_COUNT : usize = 5 ; pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn field_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn count_extra_fields (& self) -> usize { self . field_count () - Self :: FIELD_COUNT } pub fn has_extra_fields (& self) -> bool { Self :: FIELD_COUNT != self . field_count () } pub fn udt_symbol (& self) -> UTF8String { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [4 ..]) as usize ; let end = molecule :: unpack_number (& slice [8 ..]) as usize ; UTF8String :: new_unchecked (self . 0 . slice (start .. end)) } pub fn price (& self) -> Uint256 { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [8 ..]) as usize ; let end = molecule :: unpack_number (& slice [12 ..]) as usize ; Uint256 :: new_unchecked (self . 0 . slice (start .. end)) } pub fn decimal (& self) -> Uint8 { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [12 ..]) as usize ; let end = molecule :: unpack_number (& slice [16 ..]) as usize ; Uint8 :: new_unchecked (self . 0 . slice (start .. end)) } pub fn timestamp (& self) -> Uint64 { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [16 ..]) as usize ; let end = molecule :: unpack_number (& slice [20 ..]) as usize ; Uint64 :: new_unchecked (self . 0 . slice (start .. end)) } pub fn udt_type_hash (& self) -> UTF8String { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [20 ..]) as usize ; if self . has_extra_fields () { let end = molecule :: unpack_number (& slice [24 ..]) as usize ; UTF8String :: new_unchecked (self . 0 . slice (start .. end)) } else { UTF8String :: new_unchecked (self . 0 . slice (start ..)) } } pub fn as_reader < 'r > (& 'r self) -> PriceStructReader < 'r > { PriceStructReader :: new_unchecked (self . as_slice ()) } } impl molecule :: prelude :: Entity for PriceStruct { type Builder = PriceStructBuilder ; const NAME : & 'static str = "PriceStruct" ; fn new_unchecked (data : molecule :: bytes :: Bytes) -> Self { PriceStruct (data) } fn as_bytes (& self) -> molecule :: bytes :: Bytes { self . 0 . clone () } fn as_slice (& self) -> & [u8] { & self . 0 [..] } fn from_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { PriceStructReader :: from_slice (slice) . map (| reader | reader . to_entity ()) } fn from_compatible_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { PriceStructReader :: from_compatible_slice (slice) . map (| reader | reader . to_entity ()) } fn new_builder () -> Self :: Builder { :: core :: default :: Default :: default () } fn as_builder (self) -> Self :: Builder { Self :: new_builder () . udt_symbol (self . udt_symbol ()) . price (self . price ()) . decimal (self . decimal ()) . timestamp (self . timestamp ()) . udt_type_hash (self . udt_type_hash ()) } }
# [derive (Clone , Copy)] pub struct PriceStructReader < 'r > (& 'r [u8]) ; impl < 'r > :: core :: fmt :: LowerHex for PriceStructReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl < 'r > :: core :: fmt :: Debug for PriceStructReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl < 'r > :: core :: fmt :: Display for PriceStructReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} {{ " , Self :: NAME) ? ; write ! (f , "{}: {}" , "udt_symbol" , self . udt_symbol ()) ? ; write ! (f , ", {}: {}" , "price" , self . price ()) ? ; write ! (f , ", {}: {}" , "decimal" , self . decimal ()) ? ; write ! (f , ", {}: {}" , "timestamp" , self . timestamp ()) ? ; write ! (f , ", {}: {}" , "udt_type_hash" , self . udt_type_hash ()) ? ; let extra_count = self . count_extra_fields () ; if extra_count != 0 { write ! (f , ", .. ({} fields)" , extra_count) ? ; } write ! (f , " }}") } } impl < 'r > PriceStructReader < 'r > { pub const FIELD_COUNT : usize = 5 ; pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn field_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn count_extra_fields (& self) -> usize { self . field_count () - Self :: FIELD_COUNT } pub fn has_extra_fields (& self) -> bool { Self :: FIELD_COUNT != self . field_count () } pub fn udt_symbol (& self) -> UTF8StringReader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [4 ..]) as usize ; let end = molecule :: unpack_number (& slice [8 ..]) as usize ; UTF8StringReader :: new_unchecked (& self . as_slice () [start .. end]) } pub fn price (& self) -> Uint256Reader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [8 ..]) as usize ; let end = molecule :: unpack_number (& slice [12 ..]) as usize ; Uint256Reader :: new_unchecked (& self . as_slice () [start .. end]) } pub fn decimal (& self) -> Uint8Reader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [12 ..]) as usize ; let end = molecule :: unpack_number (& slice [16 ..]) as usize ; Uint8Reader :: new_unchecked (& self . as_slice () [start .. end]) } pub fn timestamp (& self) -> Uint64Reader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [16 ..]) as usize ; let end = molecule :: unpack_number (& slice [20 ..]) as usize ; Uint64Reader :: new_unchecked (& self . as_slice () [start .. end]) } pub fn udt_type_hash (& self) -> UTF8StringReader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [20 ..]) as usize ; if self . has_extra_fields () { let end = molecule :: unpack_number (& slice [24 ..]) as usize ; UTF8StringReader :: new_unchecked (& self . as_slice () [start .. end]) } else { UTF8StringReader :: new_unchecked (& self . as_slice () [start ..]) } } } impl < 'r > molecule :: prelude :: Reader < 'r > for PriceStructReader < 'r > { type Entity = PriceStruct ; const NAME : & 'static str = "PriceStructReader" ; fn to_entity (& self) -> Self :: Entity { Self :: Entity :: new_unchecked (self . as_slice () . to_owned () . into ()) } fn new_unchecked (slice : & 'r [u8]) -> Self { PriceStructReader (slice) } fn as_slice (& self) -> & 'r [u8] { self . 0 } fn verify (slice : & [u8] , compatible : bool) -> molecule :: error :: VerificationResult < () > { use molecule :: verification_error as ve ; let slice_len = slice . len () ; if slice_len < molecule :: NUMBER_SIZE { return ve ! (Self , HeaderIsBroken , molecule :: NUMBER_SIZE , slice_len) ; } let total_size = molecule :: unpack_number (slice) as usize ; if slice_len != total_size { return ve ! (Self , TotalSizeNotMatch , total_size , slice_len) ; } if slice_len < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , HeaderIsBroken , molecule :: NUMBER_SIZE * 2 , slice_len) ; } let offset_first = molecule :: unpack_number (& slice [molecule :: NUMBER_SIZE ..]) as usize ; if offset_first % molecule :: NUMBER_SIZE != 0 || offset_first < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , OffsetsNotMatch) ; } if slice_len < offset_first { return ve ! (Self , HeaderIsBroken , offset_first , slice_len) ; } let field_count = offset_first / molecule :: NUMBER_SIZE - 1 ; if field_count < Self :: FIELD_COUNT { return ve ! (Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count) ; } else if ! compatible && field_count > Self :: FIELD_COUNT { return ve ! (Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count) ; } ; let mut offsets : Vec < usize > = slice [molecule :: NUMBER_SIZE .. offset_first] . chunks_exact (molecule :: NUMBER_SIZE) . map (| x | molecule :: unpack_number (x) as usize) . collect () ; offsets . push (total_size) ; if offsets . windows (2) . any (| i | i [0] > i [1]) { return ve ! (Self , OffsetsNotMatch) ; } UTF8StringReader :: verify (& slice [offsets [0] .. offsets [1]] , compatible) ? ; Uint256Reader :: verify (& slice [offsets [1] .. offsets [2]] , compatible) ? ; Uint8Reader :: verify (& slice [offsets [2] .. offsets [3]] , compatible) ? ; Uint64Reader :: verify (& slice [offsets [3] .. offsets [4]] , compatible) ? ; UTF8StringReader :: verify (& slice [offsets [4] .. offsets [5]] , compatible) ? ; Ok (()) } }
# [derive (Clone , Debug , Default)] pub struct PriceStructBuilder { pub (crate) udt_symbol : UTF8String , pub (crate) price : Uint256 , pub (crate) decimal : Uint8 , pub (crate) timestamp : Uint64 , pub (crate) udt_type_hash : UTF8String , } impl PriceStructBuilder { pub const FIELD_COUNT : usize = 5 ; pub fn udt_symbol (mut self , v : UTF8String) -> Self { self . udt_symbol = v ; self } pub fn price (mut self , v : Uint256) -> Self { self . price = v ; self } pub fn decimal (mut self , v : Uint8) -> Self { self . decimal = v ; self } pub fn timestamp (mut self , v : Uint64) -> Self { self . timestamp = v ; self } pub fn udt_type_hash (mut self , v : UTF8String) -> Self { self . udt_type_hash = v ; self } } impl molecule :: prelude :: Builder for PriceStructBuilder { type Entity = PriceStruct ; const NAME : & 'static str = "PriceStructBuilder" ; fn expected_length (& self) -> usize { molecule :: NUMBER_SIZE * (Self :: FIELD_COUNT + 1) + self . udt_symbol . as_slice () . len () + self . price . as_slice () . len () + self . decimal . as_slice () . len () + self . timestamp . as_slice () . len () + self . udt_type_hash . as_slice () . len () } fn write < W : molecule :: io :: Write > (& self , writer : & mut W) -> molecule :: io :: Result < () > { let mut total_size = molecule :: NUMBER_SIZE * (Self :: FIELD_COUNT + 1) ; let mut offsets = Vec :: with_capacity (Self :: FIELD_COUNT) ; offsets . push (total_size) ; total_size += self . udt_symbol . as_slice () . len () ; offsets . push (total_size) ; total_size += self . price . as_slice () . len () ; offsets . push (total_size) ; total_size += self . decimal . as_slice () . len () ; offsets . push (total_size) ; total_size += self . timestamp . as_slice () . len () ; offsets . push (total_size) ; total_size += self . udt_type_hash . as_slice () . len () ; writer . write_all (& molecule :: pack_number (total_size as molecule :: Number)) ? ; for offset in offsets . into_iter () { writer . write_all (& molecule :: pack_number (offset as molecule :: Number)) ? ; } writer . write_all (self . udt_symbol . as_slice ()) ? ; writer . write_all (self . price . as_slice ()) ? ; writer . write_all (self . decimal . as_slice ()) ? ; writer . write_all (self . timestamp . as_slice ()) ? ; writer . write_all (self . udt_type_hash . as_slice ()) ? ; Ok (()) } fn build (& self) -> Self :: Entity { let mut inner = Vec :: with_capacity (self . expected_length ()) ; self . write (& mut inner) . unwrap_or_else (| _ | panic ! ("{} build should be ok" , Self :: NAME)) ; PriceStruct :: new_unchecked (inner . into ()) } }
# [derive (Clone)] pub struct PriceDataVec (molecule :: bytes :: Bytes) ; impl :: core :: fmt :: LowerHex for PriceDataVec { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl :: core :: fmt :: Debug for PriceDataVec { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl :: core :: fmt :: Display for PriceDataVec { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} [" , Self :: NAME) ? ; for i in 0 .. self . len () { if i == 0 { write ! (f , "{}" , self . get_unchecked (i)) ? ; } else { write ! (f , ", {}" , self . get_unchecked (i)) ? ; } } write ! (f , "]") } } impl :: core :: default :: Default for PriceDataVec { fn default () -> Self { let v = molecule :: bytes :: Bytes :: from_static (& Self :: DEFAULT_VALUE) ; PriceDataVec :: new_unchecked (v) } } impl PriceDataVec { const DEFAULT_VALUE : [u8 ; 4] = [4 , 0 , 0 , 0 ,] ; pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn item_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn len (& self) -> usize { self . item_count () } pub fn is_empty (& self) -> bool { self . len () == 0 } pub fn get (& self , idx : usize) -> Option < PriceStruct > { if idx >= self . len () { None } else { Some (self . get_unchecked (idx)) } } pub fn get_unchecked (& self , idx : usize) -> PriceStruct { let slice = self . as_slice () ; let start_idx = molecule :: NUMBER_SIZE * (1 + idx) ; let start = molecule :: unpack_number (& slice [start_idx ..]) as usize ; if idx == self . len () - 1 { PriceStruct :: new_unchecked (self . 0 . slice (start ..)) } else { let end_idx = start_idx + molecule :: NUMBER_SIZE ; let end = molecule :: unpack_number (& slice [end_idx ..]) as usize ; PriceStruct :: new_unchecked (self . 0 . slice (start .. end)) } } pub fn as_reader < 'r > (& 'r self) -> PriceDataVecReader < 'r > { PriceDataVecReader :: new_unchecked (self . as_slice ()) } } impl molecule :: prelude :: Entity for PriceDataVec { type Builder = PriceDataVecBuilder ; const NAME : & 'static str = "PriceDataVec" ; fn new_unchecked (data : molecule :: bytes :: Bytes) -> Self { PriceDataVec (data) } fn as_bytes (& self) -> molecule :: bytes :: Bytes { self . 0 . clone () } fn as_slice (& self) -> & [u8] { & self . 0 [..] } fn from_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { PriceDataVecReader :: from_slice (slice) . map (| reader | reader . to_entity ()) } fn from_compatible_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { PriceDataVecReader :: from_compatible_slice (slice) . map (| reader | reader . to_entity ()) } fn new_builder () -> Self :: Builder { :: core :: default :: Default :: default () } fn as_builder (self) -> Self :: Builder { Self :: new_builder () . extend (self . into_iter ()) } }
# [derive (Clone , Copy)] pub struct PriceDataVecReader < 'r > (& 'r [u8]) ; impl < 'r > :: core :: fmt :: LowerHex for PriceDataVecReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl < 'r > :: core :: fmt :: Debug for PriceDataVecReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl < 'r > :: core :: fmt :: Display for PriceDataVecReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} [" , Self :: NAME) ? ; for i in 0 .. self . len () { if i == 0 { write ! (f , "{}" , self . get_unchecked (i)) ? ; } else { write ! (f , ", {}" , self . get_unchecked (i)) ? ; } } write ! (f , "]") } } impl < 'r > PriceDataVecReader < 'r > { pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn item_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn len (& self) -> usize { self . item_count () } pub fn is_empty (& self) -> bool { self . len () == 0 } pub fn get (& self , idx : usize) -> Option < PriceStructReader < 'r > > { if idx >= self . len () { None } else { Some (self . get_unchecked (idx)) } } pub fn get_unchecked (& self , idx : usize) -> PriceStructReader < 'r > { let slice = self . as_slice () ; let start_idx = molecule :: NUMBER_SIZE * (1 + idx) ; let start = molecule :: unpack_number (& slice [start_idx ..]) as usize ; if idx == self . len () - 1 { PriceStructReader :: new_unchecked (& self . as_slice () [start ..]) } else { let end_idx = start_idx + molecule :: NUMBER_SIZE ; let end = molecule :: unpack_number (& slice [end_idx ..]) as usize ; PriceStructReader :: new_unchecked (& self . as_slice () [start .. end]) } } } impl < 'r > molecule :: prelude :: Reader < 'r > for PriceDataVecReader < 'r > { type Entity = PriceDataVec ; const NAME : & 'static str = "PriceDataVecReader" ; fn to_entity (& self) -> Self :: Entity { Self :: Entity :: new_unchecked (self . as_slice () . to_owned () . into ()) } fn new_unchecked (slice : & 'r [u8]) -> Self { PriceDataVecReader (slice) } fn as_slice (& self) -> & 'r [u8] { self . 0 } fn verify (slice : & [u8] , compatible : bool) -> molecule :: error :: VerificationResult < () > { use molecule :: verification_error as ve ; let slice_len = slice . len () ; if slice_len < molecule :: NUMBER_SIZE { return ve ! (Self , HeaderIsBroken , molecule :: NUMBER_SIZE , slice_len) ; } let total_size = molecule :: unpack_number (slice) as usize ; if slice_len != total_size { return ve ! (Self , TotalSizeNotMatch , total_size , slice_len) ; } if slice_len == molecule :: NUMBER_SIZE { return Ok (()) ; } if slice_len < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , TotalSizeNotMatch , molecule :: NUMBER_SIZE * 2 , slice_len) ; } let offset_first = molecule :: unpack_number (& slice [molecule :: NUMBER_SIZE ..]) as usize ; if offset_first % molecule :: NUMBER_SIZE != 0 || offset_first < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , OffsetsNotMatch) ; } if slice_len < offset_first { return ve ! (Self , HeaderIsBroken , offset_first , slice_len) ; } let mut offsets : Vec < usize > = slice [molecule :: NUMBER_SIZE .. offset_first] . chunks_exact (molecule :: NUMBER_SIZE) . map (| x | molecule :: unpack_number (x) as usize) . collect () ; offsets . push (total_size) ; if offsets . windows (2) . any (| i | i [0] > i [1]) { return ve ! (Self , OffsetsNotMatch) ; } for pair in offsets . windows (2) { let start = pair [0] ; let end = pair [1] ; PriceStructReader :: verify (& slice [start .. end] , compatible) ? ; } Ok (()) } }
# [derive (Clone , Debug , Default)] pub struct PriceDataVecBuilder (pub (crate) Vec < PriceStruct >) ; impl PriceDataVecBuilder { pub fn set (mut self , v : Vec < PriceStruct >) -> Self { self . 0 = v ; self } pub fn push (mut self , v : PriceStruct) -> Self { self . 0 . push (v) ; self } pub fn extend < T : :: core :: iter :: IntoIterator < Item = PriceStruct >> (mut self , iter : T) -> Self { for elem in iter { self . 0 . push (elem) ; } self } pub fn replace (& mut self , index : usize , v : PriceStruct) -> Option < PriceStruct > { self . 0 . get_mut (index) . map (| item | :: core :: mem :: replace (item , v)) } } impl molecule :: prelude :: Builder for PriceDataVecBuilder { type Entity = PriceDataVec ; const NAME : & 'static str = "PriceDataVecBuilder" ; fn expected_length (& self) -> usize { molecule :: NUMBER_SIZE * (self . 0 . len () + 1) + self . 0 . iter () . map (| inner | inner . as_slice () . len ()) . sum :: < usize > () } fn write < W : molecule :: io :: Write > (& self , writer : & mut W) -> molecule :: io :: Result < () > { let item_count = self . 0 . len () ; if item_count == 0 { writer . write_all (& molecule :: pack_number (molecule :: NUMBER_SIZE as molecule :: Number ,)) ? ; } else { let (total_size , offsets) = self . 0 . iter () . fold ((molecule :: NUMBER_SIZE * (item_count + 1) , Vec :: with_capacity (item_count) ,) , | (start , mut offsets) , inner | { offsets . push (start) ; (start + inner . as_slice () . len () , offsets) } ,) ; writer . write_all (& molecule :: pack_number (total_size as molecule :: Number)) ? ; for offset in offsets . into_iter () { writer . write_all (& molecule :: pack_number (offset as molecule :: Number)) ? ; } for inner in self . 0 . iter () { writer . write_all (inner . as_slice ()) ? ; } } Ok (()) } fn build (& self) -> Self :: Entity { let mut inner = Vec :: with_capacity (self . expected_length ()) ; self . write (& mut inner) . unwrap_or_else (| _ | panic ! ("{} build should be ok" , Self :: NAME)) ; PriceDataVec :: new_unchecked (inner . into ()) } }
pub struct PriceDataVecIterator (PriceDataVec , usize , usize) ; impl :: core :: iter :: Iterator for PriceDataVecIterator { type Item = PriceStruct ; fn next (& mut self) -> Option < Self :: Item > { if self . 1 >= self . 2 { None } else { let ret = self . 0 . get_unchecked (self . 1) ; self . 1 += 1 ; Some (ret) } } } impl :: core :: iter :: ExactSizeIterator for PriceDataVecIterator { fn len (& self) -> usize { self . 2 - self . 1 } } impl :: core :: iter :: IntoIterator for PriceDataVec { type Item = PriceStruct ; type IntoIter = PriceDataVecIterator ; fn into_iter (self) -> Self :: IntoIter { let len = self . len () ; PriceDataVecIterator (self , 0 , len) } } impl < 'r > PriceDataVecReader < 'r > { pub fn iter < 't > (& 't self) -> PriceDataVecReaderIterator < 't , 'r > { PriceDataVecReaderIterator (& self , 0 , self . len ()) } } pub struct PriceDataVecReaderIterator < 't , 'r > (& 't PriceDataVecReader < 'r > , usize , usize) ; impl < 't : 'r , 'r > :: core :: iter :: Iterator for PriceDataVecReaderIterator < 't , 'r > { type Item = PriceStructReader < 't > ; fn next (& mut self) -> Option < Self :: Item > { if self . 1 >= self . 2 { None } else { let ret = self . 0 . get_unchecked (self . 1) ; self . 1 += 1 ; Some (ret) } } } impl < 't : 'r , 'r > :: core :: iter :: ExactSizeIterator for PriceDataVecReaderIterator < 't , 'r > { fn len (& self) -> usize { self . 2 - self . 1 } }
impl :: core :: iter :: FromIterator < PriceStruct > for PriceDataVec { fn from_iter < T : IntoIterator < Item = PriceStruct >> (iter : T) -> Self { Self :: new_builder () . extend (iter) . build () } }
# [derive (Clone)] pub struct OracleData (molecule :: bytes :: Bytes) ; impl :: core :: fmt :: LowerHex for OracleData { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl :: core :: fmt :: Debug for OracleData { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl :: core :: fmt :: Display for OracleData { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} {{ " , Self :: NAME) ? ; write ! (f , "{}: {}" , "price_data" , self . price_data ()) ? ; let extra_count = self . count_extra_fields () ; if extra_count != 0 { write ! (f , ", .. ({} fields)" , extra_count) ? ; } write ! (f , " }}") } } impl :: core :: default :: Default for OracleData { fn default () -> Self { let v = molecule :: bytes :: Bytes :: from_static (& Self :: DEFAULT_VALUE) ; OracleData :: new_unchecked (v) } } impl OracleData { const DEFAULT_VALUE : [u8 ; 12] = [12 , 0 , 0 , 0 , 8 , 0 , 0 , 0 , 4 , 0 , 0 , 0 ,] ; pub const FIELD_COUNT : usize = 1 ; pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn field_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn count_extra_fields (& self) -> usize { self . field_count () - Self :: FIELD_COUNT } pub fn has_extra_fields (& self) -> bool { Self :: FIELD_COUNT != self . field_count () } pub fn price_data (& self) -> PriceDataVec { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [4 ..]) as usize ; if self . has_extra_fields () { let end = molecule :: unpack_number (& slice [8 ..]) as usize ; PriceDataVec :: new_unchecked (self . 0 . slice (start .. end)) } else { PriceDataVec :: new_unchecked (self . 0 . slice (start ..)) } } pub fn as_reader < 'r > (& 'r self) -> OracleDataReader < 'r > { OracleDataReader :: new_unchecked (self . as_slice ()) } } impl molecule :: prelude :: Entity for OracleData { type Builder = OracleDataBuilder ; const NAME : & 'static str = "OracleData" ; fn new_unchecked (data : molecule :: bytes :: Bytes) -> Self { OracleData (data) } fn as_bytes (& self) -> molecule :: bytes :: Bytes { self . 0 . clone () } fn as_slice (& self) -> & [u8] { & self . 0 [..] } fn from_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { OracleDataReader :: from_slice (slice) . map (| reader | reader . to_entity ()) } fn from_compatible_slice (slice : & [u8]) -> molecule :: error :: VerificationResult < Self > { OracleDataReader :: from_compatible_slice (slice) . map (| reader | reader . to_entity ()) } fn new_builder () -> Self :: Builder { :: core :: default :: Default :: default () } fn as_builder (self) -> Self :: Builder { Self :: new_builder () . price_data (self . price_data ()) } }
# [derive (Clone , Copy)] pub struct OracleDataReader < 'r > (& 'r [u8]) ; impl < 'r > :: core :: fmt :: LowerHex for OracleDataReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { use molecule :: hex_string ; if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{}" , hex_string (self . as_slice ())) } } impl < 'r > :: core :: fmt :: Debug for OracleDataReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{}({:#x})" , Self :: NAME , self) } } impl < 'r > :: core :: fmt :: Display for OracleDataReader < 'r > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "{} {{ " , Self :: NAME) ? ; write ! (f , "{}: {}" , "price_data" , self . price_data ()) ? ; let extra_count = self . count_extra_fields () ; if extra_count != 0 { write ! (f , ", .. ({} fields)" , extra_count) ? ; } write ! (f , " }}") } } impl < 'r > OracleDataReader < 'r > { pub const FIELD_COUNT : usize = 1 ; pub fn total_size (& self) -> usize { molecule :: unpack_number (self . as_slice ()) as usize } pub fn field_count (& self) -> usize { if self . total_size () == molecule :: NUMBER_SIZE { 0 } else { (molecule :: unpack_number (& self . as_slice () [molecule :: NUMBER_SIZE ..]) as usize / 4) - 1 } } pub fn count_extra_fields (& self) -> usize { self . field_count () - Self :: FIELD_COUNT } pub fn has_extra_fields (& self) -> bool { Self :: FIELD_COUNT != self . field_count () } pub fn price_data (& self) -> PriceDataVecReader < 'r > { let slice = self . as_slice () ; let start = molecule :: unpack_number (& slice [4 ..]) as usize ; if self . has_extra_fields () { let end = molecule :: unpack_number (& slice [8 ..]) as usize ; PriceDataVecReader :: new_unchecked (& self . as_slice () [start .. end]) } else { PriceDataVecReader :: new_unchecked (& self . as_slice () [start ..]) } } } impl < 'r > molecule :: prelude :: Reader < 'r > for OracleDataReader < 'r > { type Entity = OracleData ; const NAME : & 'static str = "OracleDataReader" ; fn to_entity (& self) -> Self :: Entity { Self :: Entity :: new_unchecked (self . as_slice () . to_owned () . into ()) } fn new_unchecked (slice : & 'r [u8]) -> Self { OracleDataReader (slice) } fn as_slice (& self) -> & 'r [u8] { self . 0 } fn verify (slice : & [u8] , compatible : bool) -> molecule :: error :: VerificationResult < () > { use molecule :: verification_error as ve ; let slice_len = slice . len () ; if slice_len < molecule :: NUMBER_SIZE { return ve ! (Self , HeaderIsBroken , molecule :: NUMBER_SIZE , slice_len) ; } let total_size = molecule :: unpack_number (slice) as usize ; if slice_len != total_size { return ve ! (Self , TotalSizeNotMatch , total_size , slice_len) ; } if slice_len < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , HeaderIsBroken , molecule :: NUMBER_SIZE * 2 , slice_len) ; } let offset_first = molecule :: unpack_number (& slice [molecule :: NUMBER_SIZE ..]) as usize ; if offset_first % molecule :: NUMBER_SIZE != 0 || offset_first < molecule :: NUMBER_SIZE * 2 { return ve ! (Self , OffsetsNotMatch) ; } if slice_len < offset_first { return ve ! (Self , HeaderIsBroken , offset_first , slice_len) ; } let field_count = offset_first / molecule :: NUMBER_SIZE - 1 ; if field_count < Self :: FIELD_COUNT { return ve ! (Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count) ; } else if ! compatible && field_count > Self :: FIELD_COUNT { return ve ! (Self , FieldCountNotMatch , Self :: FIELD_COUNT , field_count) ; } ; let mut offsets : Vec < usize > = slice [molecule :: NUMBER_SIZE .. offset_first] . chunks_exact (molecule :: NUMBER_SIZE) . map (| x | molecule :: unpack_number (x) as usize) . collect () ; offsets . push (total_size) ; if offsets . windows (2) . any (| i | i [0] > i [1]) { return ve ! (Self , OffsetsNotMatch) ; } PriceDataVecReader :: verify (& slice [offsets [0] .. offsets [1]] , compatible) ? ; Ok (()) } }
# [derive (Clone , Debug , Default)] pub struct OracleDataBuilder { pub (crate) price_data : PriceDataVec , } impl OracleDataBuilder { pub const FIELD_COUNT : usize = 1 ; pub fn price_data (mut self , v : PriceDataVec) -> Self { self . price_data = v ; self } } impl molecule :: prelude :: Builder for OracleDataBuilder { type Entity = OracleData ; const NAME : & 'static str = "OracleDataBuilder" ; fn expected_length (& self) -> usize { molecule :: NUMBER_SIZE * (Self :: FIELD_COUNT + 1) + self . price_data . as_slice () . len () } fn write < W : molecule :: io :: Write > (& self , writer : & mut W) -> molecule :: io :: Result < () > { let mut total_size = molecule :: NUMBER_SIZE * (Self :: FIELD_COUNT + 1) ; let mut offsets = Vec :: with_capacity (Self :: FIELD_COUNT) ; offsets . push (total_size) ; total_size += self . price_data . as_slice () . len () ; writer . write_all (& molecule :: pack_number (total_size as molecule :: Number)) ? ; for offset in offsets . into_iter () { writer . write_all (& molecule :: pack_number (offset as molecule :: Number)) ? ; } writer . write_all (self . price_data . as_slice ()) ? ; Ok (()) } fn build (& self) -> Self :: Entity { let mut inner = Vec :: with_capacity (self . expected_length ()) ; self . write (& mut inner) . unwrap_or_else (| _ | panic ! ("{} build should be ok" , Self :: NAME)) ; OracleData :: new_unchecked (inner . into ()) } }