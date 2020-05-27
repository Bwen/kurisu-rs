const TYPES_NO_VALUE: &[&str] = &["bool"];
const TYPES_OPTIONAL_VALUE: &[&str] = &["Option < String >", "Option < usize >", "Option < isize >"];

#[derive(Debug, Clone)]
pub struct Arg<'a> {
    pub name: &'a str,
    pub value_type: &'a str,
    pub position: Option<i8>,
    pub doc: Option<&'a str>,
    pub short: Option<&'a str>,
    pub long: Option<&'a str>,
    pub exit: Option<fn() -> i32>,
    pub default: &'a str,
    pub value: Vec<String>,
    pub occurrences: usize,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Arg<'a> {
        Arg {
            name: "",
            value_type: "",
            position: None,
            doc: None,
            short: None,
            long: None,
            exit: None,
            default: "",
            value: Vec::new(),
            occurrences: 0,
        }
    }
}

impl<'a> PartialEq<String> for &Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        self.partial_eq_string(other)
    }
}

impl<'a> PartialEq<String> for &mut Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        self.partial_eq_string(other)
    }
}

impl<'a> PartialEq<str> for &Arg<'a> {
    fn eq(&self, other: &str) -> bool {
        self.partial_eq_string(other)
    }
}

impl<'a> Arg<'a> {
    fn partial_eq_string<S: AsRef<str>>(&self, value: S) -> bool {
        let value = value.as_ref();
        if !value.starts_with('-') {
            return false;
        }

        if value.starts_with("--") && self.long.is_some() {
            return value.starts_with(&format!("--{}", self.long.unwrap()));
        }

        if value.starts_with('-') && self.short.is_some() {
            return value.starts_with(&format!("-{}", self.short.unwrap()));
        }

        false
    }

    pub fn is_value_required(&self) -> bool {
        !TYPES_NO_VALUE.contains(&self.value_type) && !TYPES_OPTIONAL_VALUE.contains(&self.value_type)
    }

    pub fn is_value_none(&self) -> bool {
        TYPES_NO_VALUE.contains(&self.value_type)
    }

    pub fn set_value(&'_ mut self, args: &[String], positions: &Vec<i8>) {
        // TODO: What to do with Optional values?
        // TODO: Handle repetitive short flags such as -vvv (Occurrences)
        // TODO: short flags that are limited 1 char... can have a value in the same word... grep -iC4 file
        // TODO: support comma delimiter args for Vec? -o test1,test2,test3

        let mut pos = 1;
        let mut options_ended = false;
        for (i, arg) in args.iter().enumerate() {
            // We only drop the first --
            if !options_ended && arg.len() == 2 && arg == "--" {
                options_ended = true;
                continue;
            }

            if self.eq(arg) {
                if arg.contains('=') {
                    let value: Vec<&str> = arg.split('=').collect();
                    self.occurrences += 1;
                    self.value.push(value[1].to_owned());
                    continue;
                }

                self.occurrences += 1;
                self.value.push(String::from("true"));
            } else if options_ended || !arg.starts_with('-') || (arg.starts_with('-') && arg.len() == 1) {
                if let Some(position) = self.position {
                    if position != pos && position != 0 && position != -1 {
                        pos += 1;
                        continue;
                    }

                    // If the arg is infinite but another arg has this position
                    if position == 0 && (positions.contains(&pos) || (positions.contains(&-1) && (i + 1) == args.len())) {
                        pos += 1;
                        continue;
                    }

                    // If we seek the last argument position
                    if position == -1 && (i + 1) != args.len() {
                        pos += 1;
                        continue;
                    }

                    self.occurrences += 1;
                    self.value.push(arg.clone());
                    pos += 1;
                }
            }
        }

        if !self.value.is_empty() {
            return;
        }

        // TODO: Fall back to check environment variables with the same name capitalized,
        // Maybe it should be mentioned in the annotation as to WHICH env var is attached to which field, to allow branding MYPROGRAM_PATH_TO_SOMETHING
        // Or... Should it? field mysql_host for example would allow a quick access to environment variables that dont require branding...
        // Optional branding annotation?

        self.value.push(self.default.to_string());
    }
}
