// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod mk_48 {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EntityType {
  ArleighBurke = 0,
  Bismarck = 1,
  Clemenceau = 2,
  Fletcher = 3,
  G5 = 4,
  Iowa = 5,
  Kolkata = 6,
  Osa = 7,
  Yasen = 8,
  Zubr = 9,

}

pub const ENUM_MIN_ENTITY_TYPE: i8 = 0;
pub const ENUM_MAX_ENTITY_TYPE: i8 = 9;

impl<'a> flatbuffers::Follow<'a> for EntityType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for EntityType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const EntityType;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const EntityType;
    unsafe { *p }
  }
}

impl flatbuffers::Push for EntityType {
    type Output = EntityType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<EntityType>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ENTITY_TYPE:[EntityType; 10] = [
  EntityType::ArleighBurke,
  EntityType::Bismarck,
  EntityType::Clemenceau,
  EntityType::Fletcher,
  EntityType::G5,
  EntityType::Iowa,
  EntityType::Kolkata,
  EntityType::Osa,
  EntityType::Yasen,
  EntityType::Zubr
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_ENTITY_TYPE:[&'static str; 10] = [
    "ArleighBurke",
    "Bismarck",
    "Clemenceau",
    "Fletcher",
    "G5",
    "Iowa",
    "Kolkata",
    "Osa",
    "Yasen",
    "Zubr"
];

pub fn enum_name_entity_type(e: EntityType) -> &'static str {
  let index = e as i8;
  ENUM_NAMES_ENTITY_TYPE[index as usize]
}

// struct Vector2f, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2f {
  x_: f32,
  y_: f32,
} // pub struct Vector2f
impl flatbuffers::SafeSliceAccess for Vector2f {}
impl<'a> flatbuffers::Follow<'a> for Vector2f {
  type Inner = &'a Vector2f;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vector2f>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vector2f {
  type Inner = &'a Vector2f;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vector2f>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vector2f {
    type Output = Vector2f;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Vector2f as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Vector2f {
    type Output = Vector2f;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Vector2f as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Vector2f {
  pub fn new<'a>(_x: f32, _y: f32) -> Self {
    Vector2f {
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

    }
  }
  pub fn x<'a>(&'a self) -> f32 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> f32 {
    self.y_.from_little_endian()
  }
}

// struct Transform, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
  altitude_: i8,
  padding0__: u8,
  angle_: u16,
  position_: Vector2f,
  velocity_: i16,
  padding1__: u16,
} // pub struct Transform
impl flatbuffers::SafeSliceAccess for Transform {}
impl<'a> flatbuffers::Follow<'a> for Transform {
  type Inner = &'a Transform;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Transform>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Transform {
  type Inner = &'a Transform;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Transform>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Transform {
    type Output = Transform;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Transform as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Transform {
    type Output = Transform;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Transform as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Transform {
  pub fn new<'a>(_altitude: i8, _angle: u16, _position: &'a Vector2f, _velocity: i16) -> Self {
    Transform {
      altitude_: _altitude.to_little_endian(),
      angle_: _angle.to_little_endian(),
      position_: *_position,
      velocity_: _velocity.to_little_endian(),

      padding0__: 0,
      padding1__: 0,
    }
  }
  pub fn altitude<'a>(&'a self) -> i8 {
    self.altitude_.from_little_endian()
  }
  pub fn angle<'a>(&'a self) -> u16 {
    self.angle_.from_little_endian()
  }
  pub fn position<'a>(&'a self) -> &'a Vector2f {
    &self.position_
  }
  pub fn velocity<'a>(&'a self) -> i16 {
    self.velocity_.from_little_endian()
  }
}

// struct Guidance, aligned to 2
#[repr(C, align(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Guidance {
  angle_: u16,
  submerge_: bool,
  padding0__: u8,
  velocity_: i16,
} // pub struct Guidance
impl flatbuffers::SafeSliceAccess for Guidance {}
impl<'a> flatbuffers::Follow<'a> for Guidance {
  type Inner = &'a Guidance;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Guidance>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Guidance {
  type Inner = &'a Guidance;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Guidance>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Guidance {
    type Output = Guidance;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Guidance as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Guidance {
    type Output = Guidance;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Guidance as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Guidance {
  pub fn new<'a>(_angle: u16, _submerge: bool, _velocity: i16) -> Self {
    Guidance {
      angle_: _angle.to_little_endian(),
      submerge_: _submerge.to_little_endian(),
      velocity_: _velocity.to_little_endian(),

      padding0__: 0,
    }
  }
  pub fn angle<'a>(&'a self) -> u16 {
    self.angle_.from_little_endian()
  }
  pub fn submerge<'a>(&'a self) -> bool {
    self.submerge_.from_little_endian()
  }
  pub fn velocity<'a>(&'a self) -> i16 {
    self.velocity_.from_little_endian()
  }
}

// struct ChunkId, aligned to 1
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChunkId {
  x_: i8,
  y_: i8,
} // pub struct ChunkId
impl flatbuffers::SafeSliceAccess for ChunkId {}
impl<'a> flatbuffers::Follow<'a> for ChunkId {
  type Inner = &'a ChunkId;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a ChunkId>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a ChunkId {
  type Inner = &'a ChunkId;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<ChunkId>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for ChunkId {
    type Output = ChunkId;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const ChunkId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b ChunkId {
    type Output = ChunkId;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const ChunkId as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl ChunkId {
  pub fn new<'a>(_x: i8, _y: i8) -> Self {
    ChunkId {
      x_: _x.to_little_endian(),
      y_: _y.to_little_endian(),

    }
  }
  pub fn x<'a>(&'a self) -> i8 {
    self.x_.from_little_endian()
  }
  pub fn y<'a>(&'a self) -> i8 {
    self.y_.from_little_endian()
  }
}

pub enum ContactOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Contact<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Contact<'a> {
    type Inner = Contact<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Contact<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Contact {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ContactArgs<'args>) -> flatbuffers::WIPOffset<Contact<'bldr>> {
      let mut builder = ContactBuilder::new(_fbb);
      if let Some(x) = args.turret_angles { builder.add_turret_angles(x); }
      if let Some(x) = args.transform { builder.add_transform(x); }
      if let Some(x) = args.reloads { builder.add_reloads(x); }
      if let Some(x) = args.guidance { builder.add_guidance(x); }
      builder.add_entity_id(args.entity_id);
      builder.add_player_id(args.player_id);
      builder.add_entity_type(args.entity_type);
      builder.add_damage(args.damage);
      builder.finish()
    }

    pub const VT_DAMAGE: flatbuffers::VOffsetT = 4;
    pub const VT_ENTITY_ID: flatbuffers::VOffsetT = 6;
    pub const VT_ENTITY_TYPE: flatbuffers::VOffsetT = 8;
    pub const VT_GUIDANCE: flatbuffers::VOffsetT = 10;
    pub const VT_PLAYER_ID: flatbuffers::VOffsetT = 12;
    pub const VT_RELOADS: flatbuffers::VOffsetT = 14;
    pub const VT_TRANSFORM: flatbuffers::VOffsetT = 16;
    pub const VT_TURRET_ANGLES: flatbuffers::VOffsetT = 18;

  #[inline]
  pub fn damage(&self) -> u8 {
    self._tab.get::<u8>(Contact::VT_DAMAGE, Some(0)).unwrap()
  }
  #[inline]
  pub fn entity_id(&self) -> u32 {
    self._tab.get::<u32>(Contact::VT_ENTITY_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn entity_type(&self) -> EntityType {
    self._tab.get::<EntityType>(Contact::VT_ENTITY_TYPE, Some(EntityType::ArleighBurke)).unwrap()
  }
  #[inline]
  pub fn guidance(&self) -> Option<&'a Guidance> {
    self._tab.get::<Guidance>(Contact::VT_GUIDANCE, None)
  }
  #[inline]
  pub fn player_id(&self) -> u16 {
    self._tab.get::<u16>(Contact::VT_PLAYER_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn reloads(&self) -> &'a [bool] {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, bool>>>(Contact::VT_RELOADS, None).map(|v| v.safe_slice()).unwrap()
  }
  #[inline]
  pub fn transform(&self) -> Option<&'a Transform> {
    self._tab.get::<Transform>(Contact::VT_TRANSFORM, None)
  }
  #[inline]
  pub fn turret_angles(&self) -> flatbuffers::Vector<'a, u16> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u16>>>(Contact::VT_TURRET_ANGLES, None).unwrap()
  }
}

pub struct ContactArgs<'a> {
    pub damage: u8,
    pub entity_id: u32,
    pub entity_type: EntityType,
    pub guidance: Option<&'a  Guidance>,
    pub player_id: u16,
    pub reloads: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , bool>>>,
    pub transform: Option<&'a  Transform>,
    pub turret_angles: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u16>>>,
}
impl<'a> Default for ContactArgs<'a> {
    #[inline]
    fn default() -> Self {
        ContactArgs {
            damage: 0,
            entity_id: 0,
            entity_type: EntityType::ArleighBurke,
            guidance: None,
            player_id: 0,
            reloads: None, // required field
            transform: None,
            turret_angles: None, // required field
        }
    }
}
pub struct ContactBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ContactBuilder<'a, 'b> {
  #[inline]
  pub fn add_damage(&mut self, damage: u8) {
    self.fbb_.push_slot::<u8>(Contact::VT_DAMAGE, damage, 0);
  }
  #[inline]
  pub fn add_entity_id(&mut self, entity_id: u32) {
    self.fbb_.push_slot::<u32>(Contact::VT_ENTITY_ID, entity_id, 0);
  }
  #[inline]
  pub fn add_entity_type(&mut self, entity_type: EntityType) {
    self.fbb_.push_slot::<EntityType>(Contact::VT_ENTITY_TYPE, entity_type, EntityType::ArleighBurke);
  }
  #[inline]
  pub fn add_guidance(&mut self, guidance: &'b  Guidance) {
    self.fbb_.push_slot_always::<&Guidance>(Contact::VT_GUIDANCE, guidance);
  }
  #[inline]
  pub fn add_player_id(&mut self, player_id: u16) {
    self.fbb_.push_slot::<u16>(Contact::VT_PLAYER_ID, player_id, 0);
  }
  #[inline]
  pub fn add_reloads(&mut self, reloads: flatbuffers::WIPOffset<flatbuffers::Vector<'b , bool>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Contact::VT_RELOADS, reloads);
  }
  #[inline]
  pub fn add_transform(&mut self, transform: &'b  Transform) {
    self.fbb_.push_slot_always::<&Transform>(Contact::VT_TRANSFORM, transform);
  }
  #[inline]
  pub fn add_turret_angles(&mut self, turret_angles: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u16>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Contact::VT_TURRET_ANGLES, turret_angles);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ContactBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ContactBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Contact<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Contact::VT_RELOADS,"reloads");
    self.fbb_.required(o, Contact::VT_TURRET_ANGLES,"turret_angles");
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum TerrainUpdateOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TerrainUpdate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TerrainUpdate<'a> {
    type Inner = TerrainUpdate<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TerrainUpdate<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TerrainUpdate {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TerrainUpdateArgs<'args>) -> flatbuffers::WIPOffset<TerrainUpdate<'bldr>> {
      let mut builder = TerrainUpdateBuilder::new(_fbb);
      if let Some(x) = args.data { builder.add_data(x); }
      if let Some(x) = args.chunk_id { builder.add_chunk_id(x); }
      builder.finish()
    }

    pub const VT_CHUNK_ID: flatbuffers::VOffsetT = 4;
    pub const VT_DATA: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn chunk_id(&self) -> Option<&'a ChunkId> {
    self._tab.get::<ChunkId>(TerrainUpdate::VT_CHUNK_ID, None)
  }
  #[inline]
  pub fn data(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(TerrainUpdate::VT_DATA, None).map(|v| v.safe_slice())
  }
}

pub struct TerrainUpdateArgs<'a> {
    pub chunk_id: Option<&'a  ChunkId>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for TerrainUpdateArgs<'a> {
    #[inline]
    fn default() -> Self {
        TerrainUpdateArgs {
            chunk_id: None,
            data: None,
        }
    }
}
pub struct TerrainUpdateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TerrainUpdateBuilder<'a, 'b> {
  #[inline]
  pub fn add_chunk_id(&mut self, chunk_id: &'b  ChunkId) {
    self.fbb_.push_slot_always::<&ChunkId>(TerrainUpdate::VT_CHUNK_ID, chunk_id);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(TerrainUpdate::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TerrainUpdateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TerrainUpdateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TerrainUpdate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum UpdateOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Update<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Update<'a> {
    type Inner = Update<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Update<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Update {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args UpdateArgs<'args>) -> flatbuffers::WIPOffset<Update<'bldr>> {
      let mut builder = UpdateBuilder::new(_fbb);
      if let Some(x) = args.terrain_updates { builder.add_terrain_updates(x); }
      builder.add_world_radius(args.world_radius);
      builder.add_score(args.score);
      if let Some(x) = args.contacts { builder.add_contacts(x); }
      builder.finish()
    }

    pub const VT_CONTACTS: flatbuffers::VOffsetT = 4;
    pub const VT_SCORE: flatbuffers::VOffsetT = 6;
    pub const VT_WORLD_RADIUS: flatbuffers::VOffsetT = 8;
    pub const VT_TERRAIN_UPDATES: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn contacts(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Contact<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Contact<'a>>>>>(Update::VT_CONTACTS, None).unwrap()
  }
  #[inline]
  pub fn score(&self) -> u32 {
    self._tab.get::<u32>(Update::VT_SCORE, Some(0)).unwrap()
  }
  #[inline]
  pub fn world_radius(&self) -> f32 {
    self._tab.get::<f32>(Update::VT_WORLD_RADIUS, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn terrain_updates(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TerrainUpdate<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<TerrainUpdate<'a>>>>>(Update::VT_TERRAIN_UPDATES, None).unwrap()
  }
}

pub struct UpdateArgs<'a> {
    pub contacts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Contact<'a >>>>>,
    pub score: u32,
    pub world_radius: f32,
    pub terrain_updates: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<TerrainUpdate<'a >>>>>,
}
impl<'a> Default for UpdateArgs<'a> {
    #[inline]
    fn default() -> Self {
        UpdateArgs {
            contacts: None, // required field
            score: 0,
            world_radius: 0.0,
            terrain_updates: None, // required field
        }
    }
}
pub struct UpdateBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> UpdateBuilder<'a, 'b> {
  #[inline]
  pub fn add_contacts(&mut self, contacts: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Contact<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Update::VT_CONTACTS, contacts);
  }
  #[inline]
  pub fn add_score(&mut self, score: u32) {
    self.fbb_.push_slot::<u32>(Update::VT_SCORE, score, 0);
  }
  #[inline]
  pub fn add_world_radius(&mut self, world_radius: f32) {
    self.fbb_.push_slot::<f32>(Update::VT_WORLD_RADIUS, world_radius, 0.0);
  }
  #[inline]
  pub fn add_terrain_updates(&mut self, terrain_updates: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<TerrainUpdate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Update::VT_TERRAIN_UPDATES, terrain_updates);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> UpdateBuilder<'a, 'b> {
    let start = _fbb.start_table();
    UpdateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Update<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Update::VT_CONTACTS,"contacts");
    self.fbb_.required(o, Update::VT_TERRAIN_UPDATES,"terrain_updates");
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum UpdatesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Updates<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Updates<'a> {
    type Inner = Updates<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Updates<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Updates {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args UpdatesArgs<'args>) -> flatbuffers::WIPOffset<Updates<'bldr>> {
      let mut builder = UpdatesBuilder::new(_fbb);
      if let Some(x) = args.updates { builder.add_updates(x); }
      builder.finish()
    }

    pub const VT_UPDATES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn updates(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Update<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Update<'a>>>>>(Updates::VT_UPDATES, None).unwrap()
  }
}

pub struct UpdatesArgs<'a> {
    pub updates: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Update<'a >>>>>,
}
impl<'a> Default for UpdatesArgs<'a> {
    #[inline]
    fn default() -> Self {
        UpdatesArgs {
            updates: None, // required field
        }
    }
}
pub struct UpdatesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> UpdatesBuilder<'a, 'b> {
  #[inline]
  pub fn add_updates(&mut self, updates: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Update<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Updates::VT_UPDATES, updates);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> UpdatesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    UpdatesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Updates<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Updates::VT_UPDATES,"updates");
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod mk48

