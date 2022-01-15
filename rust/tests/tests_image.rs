// Tests with an image buffer

#[cfg(test)]
#[cfg(feature = "image")]
mod image {
    use ::image::{Luma, open};
    use contour_tracing::image::single_l8_to_paths;

    const PATH: &str = "tests/images/";

    #[test]
    fn single_l8_to_paths_001() {
        let mut buffer = open(PATH.to_owned() + "001.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "");
    }

    #[test]
    fn single_l8_to_paths_002() {
        let mut buffer = open(PATH.to_owned() + "002.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M0 0H1V1H0Z");
    }

    #[test]
    fn single_l8_to_paths_003() {
        let mut buffer = open(PATH.to_owned() + "003.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), false), "M0 0H1V1H0");
    }

    #[test]
    fn single_l8_to_paths_004() {
        let mut buffer = open(PATH.to_owned() + "004.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M0 0H3V1H0Z");
    }

    #[test]
    fn single_l8_to_paths_005() {
        let mut buffer = open(PATH.to_owned() + "005.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M0 0H1V3H0Z");
    }

    #[test]
    fn single_l8_to_paths_006() {
        let mut buffer = open(PATH.to_owned() + "006.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M1 0H2V1H3V2H2V3H1V2H0V1H1Z");
    }

    #[test]
    fn single_l8_to_paths_007() {
        let mut buffer = open(PATH.to_owned() + "007.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M0 0H1V1H0ZM2 0H3V1H2ZM0 2H1V3H0ZM2 2H3V3H2Z");
    }

    #[test]
    fn single_l8_to_paths_008() {
        let mut buffer = open(PATH.to_owned() + "008.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), true), "M0 0H3V3H0ZM1 1V2H2V1Z");
    }

    #[test]
    fn single_l8_to_paths_009() {
        let mut buffer = open(PATH.to_owned() + "009.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), false), "M0 0H2V1H1V2H0M3 0H6V1H3M7 0H9V2H8V1H7M3 2H6V3H3M0 3H1V6H0M2 3H3V6H2M6 3H7V6H6M8 3H9V6H8M3 6H6V7H3M0 7H1V8H2V9H0M8 7H9V9H7V8H8M3 8H6V9H3");
    }

    #[test]
    fn single_l8_to_paths_010() {
        let mut buffer = open(PATH.to_owned() + "010.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), false), "M10 1H14V2H13V3H12V4H11V5H10V6H9V8H10V9H11V10H12V11H8V10H7V9H6V8H5V7H4V6H3V5H2V4H1V3H0V2H4V3H5V4H6V5H7V4H8V3H9V2H10");
    }

    #[test]
    fn single_l8_to_paths_011() {
        let mut buffer = open(PATH.to_owned() + "011.png").unwrap().to_luma8();
        assert_eq!(single_l8_to_paths(&mut buffer, Luma([255]), false), "M0 0H51V26H0M1 1V25H50V1M2 2H49V24H2M3 3V23H48V3M41 4H46V5H47V10H46V11H41V10H40V5H41M4 5H9V10H4M19 5H22V6H21V7H22V6H23V9H22V8H21V9H22V10H19V9H20V8H19V9H18V6H19V7H20V6H19M28 5H33V10H28M35 5H38V6H37V7H36V6H35M42 5V6H43V7H44V6H45V5M5 6V9H8V6M10 6H13V9H10M15 6H16V7H17V8H16V9H15V8H14V7H15M24 6H25V7H24M26 6H27V7H26M29 6V7H30V6M31 6V7H32V6M34 6H35V7H36V8H35V9H34M38 6H39V9H38V8H37V7H38M41 6V9H42V8H43V7H42V6M45 6V7H44V8H45V9H46V6M6 7H7V8H6M11 7V8H12V7M25 7H26V8H25M30 7V8H31V7M24 8H25V9H24M26 8H27V9H26M29 8V9H30V8M31 8V9H32V8M36 8H37V9H38V10H35V9H36M43 8V9H42V10H45V9H44V8M22 12H41V22H22M4 13H11V19H6V15H9V17H8V16H7V18H10V14H5V20H12V13H21V21H14V15H19V19H16V17H17V18H18V16H15V20H20V14H13V21H4M23 13V21H32V14H39V20H34V16H37V18H36V17H35V19H38V15H33V21H40V13H31V20H24V14H29V18H26V16H27V17H28V15H25V19H30V13");
    }
}
