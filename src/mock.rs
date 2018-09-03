// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use error::Error;
use gossip::Event;
use hash::Hash;
use id::{PublicId, SecretId};
use meta_vote::MetaVote;
use network_event::NetworkEvent;
use rand::{Rand, Rng};
use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::path::Path;

const NAMES: &[&str] = &[
    "Alice", "Bob", "Carol", "Dave", "Eric", "Fred", "Gina", "Hank", "Iris", "Judy", "Kent",
    "Lucy", "Mike", "Nina", "Oran", "Paul", "Quin", "Rose", "Stan", "Tina",
];

/// **NOT FOR PRODUCTION USE**: Mock signature type.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub struct Signature(String);

/// **NOT FOR PRODUCTION USE**: Mock type implementing `PublicId` and `SecretId` traits.  For
/// non-mocks, these two traits must be implemented by two separate types; a public key and secret
/// key respectively.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PeerId {
    id: String,
}

impl PeerId {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Debug for PeerId {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.id)
    }
}

impl PublicId for PeerId {
    type Signature = Signature;
    fn verify_signature(&self, _signature: &Self::Signature, _data: &[u8]) -> bool {
        true
    }
}

impl SecretId for PeerId {
    type PublicId = PeerId;
    fn public_id(&self) -> &Self::PublicId {
        &self
    }
    fn sign_detached(&self, _data: &[u8]) -> Signature {
        Signature(format!("of {:?}", self))
    }
}

/// **NOT FOR PRODUCTION USE**: Mock type implementing `NetworkEvent` trait.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Transaction(String);

impl Transaction {
    pub fn new(id: &str) -> Self {
        Transaction(id.to_string())
    }
}

impl NetworkEvent for Transaction {}

impl Display for Transaction {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Transaction({})", self.0)
    }
}

impl Debug for Transaction {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Rand for Transaction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Transaction(rng.gen_ascii_chars().take(5).collect())
    }
}

/// **NOT FOR PRODUCTION USE**: Returns a collection of mock node IDs with human-readable names.
pub fn create_ids(count: usize) -> Vec<PeerId> {
    assert!(count <= names_len());
    NAMES.iter().take(count).cloned().map(PeerId::new).collect()
}

pub fn names_len() -> usize {
    NAMES.len()
}

#[allow(unused)]
pub struct ParsedContent {
    pub(crate) events: BTreeMap<Hash, Event<Transaction, PeerId>>,
    pub(crate) events_order: Vec<Hash>,
    pub(crate) meta_votes: BTreeMap<Hash, BTreeMap<PeerId, Vec<MetaVote>>>,
}

pub fn parse_dot_file(_filename: &Path) -> Result<ParsedContent, Error> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dot_parser_fails_with_unexisting_path() {
        let file = Path::new("tests/dot_input/does_not_exist.dot");
        assert!(!file.exists());
        assert!(parse_dot_file(file).is_err());
    }

    #[test]
    fn dot_parser_fails_with_invalid_syntax() {
        let file = Path::new("tests/dot_input/invalid_syntax.dot");
        assert!(file.exists());
        assert!(parse_dot_file(file).is_err());
    }

    #[test]
    fn dot_parser_fails_with_no_graph() {
        let file = Path::new("tests/dot_input/no_graph.dot");
        assert!(file.exists());
        assert!(parse_dot_file(file).is_err());
    }

    #[test]
    fn dot_parser_succeeds_with_empty_graph() {
        let file = Path::new("tests/dot_input/empty_graph.dot");
        assert!(file.exists());
        let parsed = unwrap!(parse_dot_file(file));
        assert!(parsed.events.is_empty());
        assert!(parsed.events_order.is_empty());
        assert!(parsed.meta_votes.is_empty());
    }

}
