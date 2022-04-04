use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, sqlx::Type)]
#[sqlx(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    Pending,
    Active,
    Onhold,
    Done,
    Dropped,
    Snoozed,
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaskStatus::*;
        match s
            // Be forgiving with added punctuation or capitalization.
            .replace(|ch: char| !ch.is_ascii_alphabetic(), "")
            .to_ascii_lowercase()
            .as_str()
        {
            "pending" => Ok(Pending),
            "active" => Ok(Active),
            "onhold" => Ok(Onhold),
            "done" => Ok(Done),
            "dropped" => Ok(Dropped),
            "snoozed" => Ok(Snoozed),
            bad => Err(bad.to_string()),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TaskStatus::*;
        let s = match self {
            Pending => "PENDING",
            Active => "ACTIVE",
            Onhold => "ONHOLD",
            Done => "DONE",
            Dropped => "DROPPED",
            Snoozed => "SNOOZED",
        };
        f.write_str(s)
    }
}

// impl<'r> Decode<'r, Sqlite> for TaskStatus
// where
//     &'r str: Decode<'r, Sqlite>,
// {
//     fn decode(value: SqliteValueRef<'r>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
//         let s = <&str as Decode<Sqlite>>::decode(value)?;
//         Ok(s.parse()?)
//     }
// }

// impl Encode<'_, Sqlite> for TaskStatus {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull {
//         Encode::<Sqlite>::encode(self.to_string(), buf)
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_each_status() {
        use TaskStatus::*;
        assert_eq!(TaskStatus::from_str("pending"), Ok(Pending));
        assert_eq!(TaskStatus::from_str("active"), Ok(Active));
        assert_eq!(TaskStatus::from_str("onhold"), Ok(Onhold));
        assert_eq!(TaskStatus::from_str("done"), Ok(Done));
        assert_eq!(TaskStatus::from_str("dropped"), Ok(Dropped));
        assert_eq!(TaskStatus::from_str("snoozed"), Ok(Snoozed));
    }

    #[test]
    fn parse_odd_inputs() {
        use TaskStatus::*;
        assert_eq!(TaskStatus::from_str("peNDiNg!"), Ok(Pending));
        assert_eq!(TaskStatus::from_str("PEN-ding "), Ok(Pending));
        assert_eq!(TaskStatus::from_str("P #️⃣ ending"), Ok(Pending));
    }
}
