use crate::{core::WaitFor, Image};

const NAME: &str = "google/cloud-sdk";
const DEFAULT_TAG: &str = "353.0.0";

const HOST: &str = "0.0.0.0";
pub const BIGTABLE_PORT: u16 = 8086;
pub const DATASTORE_PORT: u16 = 8081;
pub const FIRESTORE_PORT: u16 = 8080;
pub const PUBSUB_PORT: u16 = 8085;

#[derive(Debug, Clone)]
pub struct CloudSdkArgs {
    pub host: String,
    pub port: u16,
    pub emulator: Emulator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Emulator {
    Bigtable,
    Datastore { project: String },
    Firestore,
    PubSub,
}

impl IntoIterator for CloudSdkArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let (emulator, project) = match &self.emulator {
            Emulator::Bigtable => ("bigtable", None),
            Emulator::Datastore { project } => ("datastore", Some(project)),
            Emulator::Firestore => ("firestore", None),
            Emulator::PubSub => ("pubsub", None),
        };
        let mut args = vec![
            "gcloud".to_owned(),
            "beta".to_owned(),
            "emulators".to_owned(),
            emulator.to_owned(),
            "start".to_owned(),
        ];
        if let Some(project) = project {
            args.push("--project".to_owned());
            args.push(project.to_owned());
        }
        args.push("--host-port".to_owned());
        args.push(format!("{}:{}", self.host, self.port));

        args.into_iter()
    }
}

#[derive(Debug)]
pub struct CloudSdk {
    tag: String,
    exposed_port: u16,
    ready_condition: WaitFor,
}

impl Image for CloudSdk {
    type Args = CloudSdkArgs;

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![self.ready_condition.clone()]
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![self.exposed_port]
    }
}

impl CloudSdk {
    fn new(port: u16, emulator: Emulator, ready_condition: WaitFor) -> (Self, CloudSdkArgs) {
        let tag = DEFAULT_TAG.to_owned();
        let arguments = CloudSdkArgs {
            host: HOST.to_owned(),
            port,
            emulator,
        };
        let exposed_port = port;
        (
            Self {
                tag,
                exposed_port,
                ready_condition,
            },
            arguments,
        )
    }

    pub fn bigtable() -> (Self, CloudSdkArgs) {
        Self::new(
            BIGTABLE_PORT,
            Emulator::Bigtable,
            WaitFor::message_on_stderr("[bigtable] Cloud Bigtable emulator running on"),
        )
    }

    pub fn firestore() -> (Self, CloudSdkArgs) {
        Self::new(
            FIRESTORE_PORT,
            Emulator::Firestore,
            WaitFor::message_on_stderr("[firestore] Dev App Server is now running"),
        )
    }

    pub fn datastore(project: impl Into<String>) -> (Self, CloudSdkArgs) {
        let project = project.into();
        Self::new(
            DATASTORE_PORT,
            Emulator::Datastore { project },
            WaitFor::message_on_stderr("[datastore] Dev App Server is now running"),
        )
    }

    pub fn pubsub() -> (Self, CloudSdkArgs) {
        Self::new(
            PUBSUB_PORT,
            Emulator::PubSub,
            WaitFor::message_on_stderr("[pubsub] INFO: Server started, listening on"),
        )
    }
}