extern crate speculate as other_speculate;
use other_speculate::speculate;

pub fn zero() -> u32 {
  0
}

speculate! {
  before {
    let mut i = zero();
  }

  before {
    i += 1;
  }

  after {
    i += 1;
  }

  after {
    assert_eq!(i, 6);
  }

  it works_at_level_1 {
    assert_eq!(i, zero() + 1);
    i = 5;
  }

  describe "something" {
    before {
      assert_eq!(i, zero() + 1);
      i = 1;
    }

    it works_at_level_2 {
      assert_eq!(i, 1);
      i = 4;
    }

    after {
      assert_eq!(i, 4);
      i = 5;
    }

    context "nested" {
      before {
        assert_eq!(i, 1);
        i = 2;
      }

      test works_at_level_3 {
        assert_eq!(i, 2);
        i = 3;
      }

      after {
        assert_eq!(i, 3);
        i = 4;
      }
    }

    it works_at_level_2_after_context {
      assert_eq!(i, 1);
      i = 4;
    }
  }

  it works_at_level_1_after_describe {
    assert_eq!(i, zero() + 1);
    i = 5;
  }
}

// Parsing edge cases
mod ec1 {
  use other_speculate::speculate;

  speculate! {}
}

mod ec2 {
  use other_speculate::speculate;

  speculate! {
    before {}
    it works {}
  }

  speculate! {
    // Many modules in a same scope!
    it works_again {}
  }
}

mod ec3 {
  use other_speculate::speculate;

  speculate! {
    it foo {}
  }
}

mod ec4 {
  use other_speculate::speculate;

  speculate! {
    after {}
  }
}

mod ec5 {
  use other_speculate::speculate;

  speculate! {
    before {}
    it foo {}
    after {}
  }
}

mod attributes {
  use other_speculate::speculate;

  speculate! {
    #[ignore]
    test ignore {
      assert_eq!(1, 2);
    }

    #[should_panic]
    test should_panic {
      assert_eq!(1, 2);
    }

    #[should_panic(expected = "foo")]
    test should_panic_with_foo {
      panic!("foo");
    }
  }
}
