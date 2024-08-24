use strype::{define_string_type};

#[test]
fn string_unchecked() {
  define_string_type! {
      /// Wrapper for some markdown source text
      ///
      /// All strings are valid markdown sources, so this is only a semantic wrapper.
      /// It does not have any specific checks.
      pub struct Markdown(String);
  }

  const HELLO_TITLE: Markdown<&'static str> = Markdown::new("# Hello, Strype!");

  let hello_title: Markdown = Markdown::new(String::from("# Hello, Strype!"));

  assert_eq!(hello_title.as_view(), HELLO_TITLE);
  assert_eq!(hello_title.as_str(), "# Hello, Strype!");
  assert_eq!(hello_title.into_inner(), String::from("# Hello, Strype!"));
}

#[test]
fn string_const_checked() {
  define_string_type! {
    /// Simple username, non-empty 3-20 char ascii alphanumeric trimmed string
    pub struct Username(String);

    macro username;

    check pub const UsernameError {
      #[non_empty]
      NonEmpty,
      #[ascii_trimmed]
      Trimmed,
      #[min_len(3)]
      MinLen,
      #[max_len(20)]
      MaxLen,
    }
  }

  const AUTHOR_USERNAME: Username<&'static str> = match Username::check("demurgos") {
    Ok(u) => u,
    Err(_) => panic!("`demurgos` is a valid username"),
  };

  let author_username: Username = Username::new(String::from("demurgos")).unwrap();

  assert_eq!(author_username.as_view(), AUTHOR_USERNAME);
  assert_eq!(author_username.as_str(), "demurgos");
  assert_eq!(author_username.into_inner(), String::from("demurgos"));

  assert_eq!(Username::new(""), Err(UsernameError::NonEmpty));
  assert_eq!(Username::new(" demurgos "), Err(UsernameError::Trimmed));
}

#[test]
fn string_dyn_checked() {
  define_string_type! {
    /// 6 character 8-bit RGB lowercase hex code
    pub struct Rgb8Hex(String);

    check pub dyn Rgb8HexError {
      #[non_empty]
      NonEmpty,
      #[ascii_trimmed]
      Trimmed,
      #[len(6)]
      Len,
      #[regex("^[0-9a-f]{6}$")]
      HexOnly,
    }
  }

  let red: Rgb8Hex = Rgb8Hex::new(String::from("ff0000")).unwrap();

  assert_eq!(red.as_str(), "ff0000");
  assert_eq!(red.into_inner(), String::from("ff0000"));

  assert_eq!(Rgb8Hex::new(""), Err(Rgb8HexError::NonEmpty));
  assert_eq!(Rgb8Hex::new(" ff0000 "), Err(Rgb8HexError::Trimmed));
  assert_eq!(Rgb8Hex::new("zz0000"), Err(Rgb8HexError::HexOnly));
}
