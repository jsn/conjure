use result::{error, Result};
use std::str::FromStr;
use util;

#[derive(Debug)]
pub enum Lang {
    Clojure,
    ClojureScript,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "couldn't parse language, should be clj or cljs")]
    ParseLangError,
}

impl FromStr for Lang {
    type Err = failure::Error;

    fn from_str(lang: &str) -> Result<Self> {
        match lang {
            "clj" => Ok(Lang::Clojure),
            "cljs" => Ok(Lang::ClojureScript),
            _ => Err(error(Error::ParseLangError)),
        }
    }
}

pub fn bootstrap() -> String {
    "
    (set! *print-length* 50)
    (require '#?(:clj clojure.repl, :cljs cljs.repl))
    #?(:clj (require 'clojure.stacktrace))
    (str \"Ready to evaluate \" #?(:clj \"Clojure\", :cljs \"ClojureScript\") \"!\")
    ".to_owned()
}

pub fn eval(code: &str, ns: &str, lang: &Lang) -> String {
    let wrapped = format!("(clojure.core/in-ns '{}) {}", ns, code);

    match lang {
        Lang::Clojure => format!(
            "
            (try
              (clojure.core/eval (clojure.core/read-string {{:read-cond :allow}} \"(do {})\"))
              (catch Throwable e
                (binding [*out* *err*]
                  (clojure.stacktrace/print-stack-trace e)
                  (println))))
            ",
            util::escape_quotes(&wrapped),
        ),
        Lang::ClojureScript => wrapped,
    }
}
