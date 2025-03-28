/// Define a newtype wrapper for a string.
///
/// # Minimal example
///
/// ```
/// use nype::define_string_type;
///
/// define_string_type!{
///   pub struct BlogTitle(String);
/// }
///
/// let title: BlogTitle<&'static str> = BlogTitle::new("Announcing Strype!");
/// ```
///
/// # Full example
///
/// ```
/// use nype::define_string_type;
///
/// define_string_type!{
///   pub struct BlogTitle(String);
///
///   #[error(const)]
///   pub enum BlogTitleParseError {}
/// }
///
/// let title: Result<BlogTitle<&'static str>, BlogTitleParseError> = BlogTitle::new("Announcing Strype!");
/// ```
#[macro_export]
macro_rules! define_string_type {
  // main rule:
  // 1. Main string wrapper, as a unit struct wrapping the owned string type
  // 2. (optional) Parse error, each variant is a check
  // 3. (optional) Literal macro
  (
    $(#[$struct_meta:meta])*
    $struct_vis:vis struct $struct_name:ident($inner_ty:ty);

    $(
      #[error($ck_const:ident)]
      $(#[$err_meta:meta])*
      $err_vis:vis enum $err_name:ident {
        $(
          #[$($ck_meta:tt)*]
          $ck_name:ident,
        )*
      }
    )?

    $(
      #[macro]
      $(#[$macro_meta:meta])*
      $macro_name:ident;
    )?
  ) => {
    // The new-type definition is expanded with any special cases
    // Method impls are split:
    // - conditional methods defined in macro rules (`@impl_new`)
    // - unconditional methods are defined a bit lower in this block
    $(#[$struct_meta])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    $struct_vis struct $struct_name<TyInner: ?Sized = $inner_ty>(TyInner);

    $(
      $(#[$err_meta])*
      #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
      $err_vis enum $err_name {
        $($ck_name,)*
      }
    )?

    // $(
    //   #[macro_export]
    //   macro_rules! $macro_name {
    //     ($input:expr) => {
    //       {
    //         mod string_check {
    //           #[test]
    //           fn check_macro_value() {
    //             <super::$struct_name as ::core::str::FromStr>::from_str($input).unwrap();
    //           }
    //         }
    //         $struct_name($input)
    //       }
    //     };
    //   }
    // )?

    // conitional method definition: their signature changes depending
    // if there are checks or not (fallible constructor or not)
    $crate::define_string_type!(
      @impl_new $struct_name($inner_ty)
      $($err_name($ck_const) {
        $(
          #[$($ck_meta)*]
          $ck_name,
        )*
      })?
    );

    /// Get a `&str` string slice reference to the inner value.
    impl<TyInner: ?Sized> $struct_name<TyInner> {
      pub fn as_str(&self) -> &str
        where TyInner: ::core::convert::AsRef<str>,
      {
        self.0.as_ref()
      }

      /// Get a strongly typed string slice reference.
      pub fn as_view(&self) -> &$struct_name<str>
        where TyInner: ::core::convert::AsRef<str>,
      {
        $struct_name(self.as_str()).transpose()
      }

      /// Extract the inner value our of the wrapper.
      pub fn into_inner(self) -> TyInner
        where TyInner: Sized
      {
        self.0
      }

      /// Get a reference to the inner value.
      pub const fn as_inner(&self) -> &TyInner {
        &self.0
      }
    }

    impl<'s> $struct_name<&'s str> {
      /// Specialized version of [into_inner] suitable for const context when the inner type is &str.
      ///
      /// Outside of `const` contexts, it is recommended to use `into_inner` or `as_str` directly.
      pub const fn into_inner_str(self) -> &'s str {
        self.0
      }

      /// Convert a "wrapped (str ref)" to a "(wrapped str) ref".
      pub const fn transpose(self) -> &'s $struct_name<str> {
        // get the inner `&str` ref
        let s: &'s str = self.into_inner_str();
        // convert it to a str fat pointer
        let str_ptr: *const str = core::ptr::from_ref(s);
        // cast to a wrapper<str> fat pointer (using repr-transparency)
        let wrapped_ptr: *const $struct_name<str> = str_ptr as *const $struct_name<str>;

        unsafe {
          // SAFETY:
          // The code below is equivalent to `core::mem::transmute(s)`, but works in a `const`
          // context. The safety is therefore based on `core::mem::transmute`.
          // The `Src` type is `&'s str`
          // The `Dst` type is `&'s $struct_name<str>` where `$struct_name<str>` is a
          // `repr(transparent)` wrapper for `str`.
          // Therefore `Dst` has the same representation as `Src` and transmuting is safe.
          //
          // You may also see this discussion about fat pointer casts:
          // <https://internals.rust-lang.org/t/pre-rfc-generic-pointer-casts-aka-ptr-cast-for-fat-pointers/20210>

          // We can convert because:
          // 1. `s` passes was obtained from `into_inner_str`, we know that it passws the checks
          // 2. the wrapper is repr-transparent
          // 3. `str_ptr` was obtained from `s`, a valid ref; `str_ptr` is therefore valid
          &*wrapped_ptr
        }
      }
    }

    impl $struct_name<Box<str>> {
      /// Convert a "wrapped (str box)" to a "(wrapped str) box".
      pub fn transpose(self) -> Box<$struct_name<str>> {
        // get the inner `str` box
        let s: Box<str> = self.into_inner();
        // convert it to a str fat pointer
        let str_ptr: *mut str = Box::into_raw(s);
        // cast to a wrapper<str> fat pointer (using repr-transparency)
        let wrapped_ptr: *mut $struct_name<str> = str_ptr as *mut $struct_name<str>;

        unsafe {
          // SAFETY:
          // The code below is equivalent to `core::mem::transmute(s)`, but works in a `const`
          // context. The safety is therefore based on `core::mem::transmute`.
          // The `Src` type is `&'s str`
          // The `Dst` type is `&'s $struct_name<str>` where `$struct_name<str>` is a
          // `repr(transparent)` wrapper for `str`.
          // Therefore `Dst` has the same representation as `Src` and transmuting is safe.
          //
          // You may also see this discussion about fat pointer casts:
          // <https://internals.rust-lang.org/t/pre-rfc-generic-pointer-casts-aka-ptr-cast-for-fat-pointers/20210>

          // We can convert because:
          // 1. `s` passes was obtained from `into_inner_str`, we know that it passws the checks
          // 2. the wrapper is repr-transparent
          // 3. `str_ptr` was obtained from `s`, a valid ref; `str_ptr` is therefore valid
          Box::from_raw(wrapped_ptr)
        }
      }
    }
  };

  // internal rule for method implementation in the case where there are no checks (all strings are valid)
  (@impl_new $struct_name:ident($inner_ty:ty)) => {
    impl<TyInner> $struct_name<TyInner> {
      pub const fn new(inner: TyInner) -> Self
        where TyInner: ::core::ops::Deref<Target = str>,
      {
        Self(inner)
      }
    }

    impl ::core::str::FromStr for $struct_name<$inner_ty> {
      type Err = ::core::convert::Infallible;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(<$inner_ty>::from(s)))
      }
    }
  };

  // internal rule for method implementation in the case where there are const checks (all strings are not valid)
  (
    @impl_new $struct_name:ident($inner_ty:ty)
    $err_name:ident(const) {
      $(
        #[$($ck_meta:tt)*]
        $ck_name:ident,
      )*
    }
  ) => {
    impl<TyInner> $struct_name<TyInner> {
      pub fn new(input: TyInner) -> Result<Self, $err_name>
        where TyInner: ::core::ops::Deref<Target = str>,
      {
        match $struct_name::new_ref(&*input) {
          Ok(_) => Ok(Self(input)),
          Err(e) => Err(e),
        }
      }
    }

    impl<'s> $struct_name<&'s str>
    {
      /// Build a string slice wrapper ref
      pub const fn new_ref(input: &'s str) -> Result<&'s $struct_name<str>, $err_name> {
        $(
          $crate::define_string_type!(@check $err_name::$ck_name($($ck_meta)*)(input));
        )*
        Ok(Self(input).transpose())
      }
    }

    impl $struct_name<Box<str>>
    {
      /// Build a boxed wrapped string slice wrapper
      pub fn new_box(input: Box<str>) -> Result<Box<$struct_name<str>>, $err_name> {
        match $struct_name::new_ref(&*input) {
          Ok(_) => Ok(Self(input).transpose()),
          Err(e) => Err(e),
        }
      }
    }

    impl ::core::str::FromStr for $struct_name<$inner_ty> {
      type Err = $err_name;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(<$inner_ty>::from(s))
      }
    }
  };

  // internal rule for method implementation in the case where there are dyn (non-const) checks (all strings are not valid)
  (
    @impl_new $struct_name:ident($inner_ty:ty)
    $err_name:ident(dyn) {
      $(
        #[$($ck_meta:tt)*]
        $ck_name:ident,
      )*
    }
  ) => {
    impl<TyInner> $struct_name<TyInner> {
      pub fn new(input: TyInner) -> Result<Self, $err_name>
        where TyInner: ::core::ops::Deref<Target = str>,
      {
        match $struct_name::new_ref(&*input) {
          Ok(_) => Ok(Self(input)),
          Err(e) => Err(e),
        }
      }
    }

    impl<'s> $struct_name<&'s str>
    {
      /// Specialized new for `&'s str`.
      pub fn new_ref(input: &'s str) -> Result<&'s $struct_name<str>, $err_name> {
        $(
          $crate::define_string_type!(@check $err_name::$ck_name($($ck_meta)*)(input));
        )*
        Ok(Self(input).transpose())
      }
    }

    impl ::core::str::FromStr for $struct_name<$inner_ty> {
      type Err = $err_name;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(<$inner_ty>::from(s))
      }
    }
  };

  (@check $err_name:ident::$ck_name:ident(non_empty)($input:expr)) => {
    if $input.is_empty() {
      return Err($err_name::$ck_name);
    }
  };

  (@check $err_name:ident::$ck_name:ident(ascii_trimmed)($input:expr)) => {
    if $input.trim_ascii().len() != $input.len() {
      return Err($err_name::$ck_name);
    }
  };

  (@check $err_name:ident::$ck_name:ident(min_len($l:expr))($input:expr)) => {
    if $input.len() < $l {
      return Err($err_name::$ck_name);
    }
  };

  (@check $err_name:ident::$ck_name:ident(max_len($l:expr))($input:expr)) => {
    if $input.len() > $l {
      return Err($err_name::$ck_name);
    }
  };

  (@check $err_name:ident::$ck_name:ident(len($l:expr))($input:expr)) => {
    if $input.len() != $l {
      return Err($err_name::$ck_name);
    }
  };

  (@check $err_name:ident::$ck_name:ident(regex($pattern:expr))($input:expr)) => {
    #[allow(clippy::trivial_regex)]
    static PATTERN: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
      let pat: &str = $pattern;
      match ::regex::Regex::new(pat) {
        Ok(pat) => pat,
        Err(e) => panic!("regex check {}::{} pattern {pat:?} should be valid: {e}", stringify!($err_name), stringify!($ck_name)),
      }
    });
    if !PATTERN.is_match($input) {
      return Err($err_name::$ck_name);
    }
  };
}
