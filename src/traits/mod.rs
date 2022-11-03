pub mod traits;
pub mod derive;
pub mod return_traits_with_dyn;
pub mod operator_overloading;
pub mod drop_trait;
pub mod iterators;
pub mod impl_trait;
pub mod clone;
pub mod supertraits;
pub mod overlapping_traits;

pub fn call_all(){
  traits::traits();
  derive::derive();
  return_traits_with_dyn::return_traits_with_dyn();
  operator_overloading::operator_overloading();
  drop_trait::drop_trait();
  iterators::iterators();
  impl_trait::impl_trait();
  clone::clone();
  supertraits::supertraits();
  overlapping_traits::overlapping_traits();
}