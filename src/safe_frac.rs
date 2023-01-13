use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;

use serde::{Serialize, Deserialize,de};
use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign,};
use core::fmt::Debug;
#[derive(PartialEq,PartialOrd,Clone,Copy,Debug,Default)]
pub struct SafeDouble(f64);
impl SafeDouble{
    ///#panics
    /// if the argument is outside the total order
    pub fn new(data: f64)->SafeDouble{
        if data == f64::NAN{
            panic!("data is NAN, which is non-orderable");
        }
        SafeDouble(data)
    }
    /// Constructs an ```Option<SafeDouble>``` out of the argument. Returns None if the argument is outside the total order.
    pub fn new_checked(data: f64)->Option<SafeDouble>{
        if data==f64::NAN{
            None
        } else{
            Some(SafeDouble(data))
        }
    }
    /// Constructs a ```SafeDouble``` without a validity check. Incorrectly constructed SafeDoubles may panic on calls to .cmp
    /// Comparison Operators may have surprising behavior but will not panic
    pub fn new_unckecked(data:f64)->SafeDouble{
        SafeDouble(data)
    }
    pub fn into_iner(&self)->f64{
        self.0
    }
    pub fn check(&self){
        if self.0==f64::NAN{
            panic!("Invalid state! Eneded up with NAN in a safedouble")
        }
    }
}
impl Display for SafeDouble{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl From<f64> for SafeDouble{
    fn from(k: f64) -> Self {
        SafeDouble::new(k)
    }
}
impl Into<f64> for SafeDouble{
    fn into(self) -> f64 {
        self.into_iner()
    }
}
impl Eq for SafeDouble{}
impl Ord for SafeDouble{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("One of the compared values is non-comprable")
    }
}
impl Deref for SafeDouble{
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<f64> for SafeDouble{
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}
impl Serialize for SafeDouble{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SafeDouble{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        match SafeDouble::new_checked(f64::deserialize(deserializer)?){
            Some(s)=>Ok(s),
            None=>Err(de::Error::custom("Invalid decimal"))
        }
    }
}
impl Hash for SafeDouble{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
impl Add for SafeDouble{
    type Output = SafeDouble;
    fn add(self, rhs: Self) -> Self::Output {
        SafeDouble::new(self.0+rhs.0)
    }
}
impl<T> Add<T> for SafeDouble
where
    f64: Add<T>,
    <f64 as Add<T>>:: Output: Into<f64>
{
    type Output=SafeDouble;

    fn add(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.add(rhs).into())
    }
}
impl Sub for SafeDouble{
    type Output=SafeDouble;

    fn sub(self, rhs: Self) -> Self::Output {
        SafeDouble::new(self.0-rhs.0)
    }
}
impl<T> Sub<T> for SafeDouble
where
    f64: Sub<T>,
    <f64 as Sub<T>>:: Output: Into<f64>
{
    type Output=SafeDouble;

    fn sub(self, rhs: T) -> Self::Output {
        SafeDouble::new((self.0-rhs).into())
    }
}
impl Mul for SafeDouble{
    type Output=SafeDouble;

    fn mul(self, rhs: Self) -> Self::Output {
        SafeDouble::new(self.0.mul(rhs.0))
    }
}
impl<T> Mul<T> for SafeDouble
where
    f64: Mul<T>,
    <f64 as Mul<T>>:: Output: Into<f64>
{
    type Output=SafeDouble;

    fn mul(self, rhs: T) -> Self::Output {
        SafeDouble::new((self.0*rhs).into())
    }
}
impl Div for SafeDouble{
    type Output = SafeDouble;
    fn div(self, rhs: Self) -> Self::Output {
        SafeDouble::new(self.0.div(rhs.0))
    }
}
impl<T> Div<T> for SafeDouble
where
    f64: Div<T>,
    <f64 as Div<T>>:: Output: Into<f64>
{
    type Output=SafeDouble;
    fn div(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.div(rhs).into())
    }
}
impl<T> Rem<T> for SafeDouble
where
    f64: Rem<T>,
    <f64 as Rem<T>>:: Output: Into<f64>
{
    type Output = SafeDouble;
    fn rem(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.rem(rhs).into())
    }
}
impl<T> BitAnd<T> for SafeDouble
where
    f64: BitAnd<T>,
    <f64 as BitAnd<T>>:: Output: Into<f64>
{
    type Output =  SafeDouble;
    fn bitand(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.bitand(rhs).into())
    }
}
impl<T> BitOr<T> for SafeDouble
where
    f64: BitOr<T>,
    <f64 as BitOr<T>>:: Output: Into<f64>
{
    type Output = SafeDouble;
    fn bitor(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.bitor(rhs).into())
    }
}
impl<T> BitXor<T> for SafeDouble
where
    f64: BitXor<T>,
    <f64 as BitXor<T>>:: Output:Into<f64>
{
    type Output = SafeDouble;
    fn bitxor(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.bitxor(rhs).into())       
    }
}
impl<T> Shl<T> for SafeDouble
where
    f64: Shl<T>,
    <f64 as Shl<T>>:: Output: Into<f64>
{
    type Output = SafeDouble;
    fn shl(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.shl(rhs).into())
    }
}
impl<T> Shr<T> for SafeDouble
where
    f64: Shr<T>,
    <f64 as Shr<T>>:: Output: Into<f64>
{
    type Output = SafeDouble;
    fn shr(self, rhs: T) -> Self::Output {
        SafeDouble::new(self.0.shr(rhs).into())
    }
}
impl AddAssign for SafeDouble{
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
        self.check();
    }
}
impl<T> AddAssign<T> for SafeDouble
where
    f64: AddAssign<T>
{
    fn add_assign(&mut self, rhs: T) {
        self.0.add_assign(rhs);
        self.check();
    }
}
impl SubAssign for SafeDouble{
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0);
        self.check();
    }
}
impl<T> SubAssign<T> for SafeDouble
where
    f64: SubAssign<T>
{
    fn sub_assign(&mut self, rhs: T) {
        self.0.sub_assign(rhs);
        self.check();
    }
}
impl MulAssign for SafeDouble{
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0);
        self.check();
    }
}
impl<T> MulAssign<T> for SafeDouble
where
    f64: MulAssign<T>
{
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
        self.check();
    }
}
impl DivAssign for SafeDouble{
    fn div_assign(&mut self, rhs: Self) {
        self.0.div_assign(rhs.0);
        self.check();
    }
}
impl<T> DivAssign<T> for SafeDouble
where
    f64: DivAssign<T>
{
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
        self.check();
    }
}
impl<T> RemAssign<T> for SafeDouble
where
    f64: RemAssign<T>
{
    fn rem_assign(&mut self, rhs: T) {
        self.0.rem_assign(rhs);
        self.check();
    }
}
impl<T> BitAndAssign<T> for SafeDouble
where
    f64: BitAndAssign<T>
{
    fn bitand_assign(&mut self, rhs: T) {
        self.0.bitand_assign(rhs);
        self.check();
    }
}
impl<T> BitOrAssign<T> for SafeDouble
where
    f64: BitOrAssign<T>
{
    fn bitor_assign(&mut self, rhs: T) {
        self.0.bitor_assign(rhs);
        self.check();
    }
}
impl<T> BitXorAssign<T> for SafeDouble
where
    f64: BitXorAssign<T>
{
    fn bitxor_assign(&mut self, rhs: T) {
        self.0.bitxor_assign(rhs);
        self.check();
    }
}
impl<T> ShlAssign<T> for SafeDouble
where
    f64: ShlAssign<T>
{
    fn shl_assign(&mut self, rhs: T) {
        self.0.shl_assign(rhs);
        self.check()
    }
}
impl<T> ShrAssign<T> for SafeDouble
where
    f64: ShrAssign<T>
{
    fn shr_assign(&mut self, rhs: T) {
        self.0.shr_assign(rhs);
        self.check();
    }
}