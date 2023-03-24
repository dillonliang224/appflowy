use crate::database::database_editor::DatabaseEditorTest;

use database_model::FieldRevision;
use flowy_client_sync::client_database::{DatabaseOperations, DatabaseRevisionPad};
use flowy_revision::{RevisionSnapshotData, REVISION_WRITE_INTERVAL_IN_MILLIS};
use revision_model::Revision;
use std::time::Duration;
use tokio::time::sleep;

pub enum SnapshotScript {
  WriteSnapshot,
  #[allow(dead_code)]
  AssertSnapshot {
    rev_id: i64,
    expected: Option<RevisionSnapshotData>,
  },
  AssertSnapshotContent {
    snapshot: RevisionSnapshotData,
    expected: String,
  },
  CreateField {
    field_rev: FieldRevision,
  },
  DeleteField {
    field_rev: FieldRevision,
  },
}

pub struct DatabaseSnapshotTest {
  inner: DatabaseEditorTest,
  pub current_snapshot: Option<RevisionSnapshotData>,
  pub current_revision: Option<Revision>,
}

impl DatabaseSnapshotTest {
  pub async fn new() -> Self {
    let editor_test = DatabaseEditorTest::new_grid().await;
    Self {
      inner: editor_test,
      current_snapshot: None,
      current_revision: None,
    }
  }

  pub fn grid_id(&self) -> String {
    self.view_id.clone()
  }

  pub async fn grid_pad(&self) -> DatabaseRevisionPad {
    self.editor.database_pad().read().await.clone()
  }

  pub async fn run_scripts(&mut self, scripts: Vec<SnapshotScript>) {
    for script in scripts {
      self.run_script(script).await;
    }
  }

  pub async fn get_latest_snapshot(&self) -> Option<RevisionSnapshotData> {
    self.editor.rev_manager().read_snapshot(None).await.unwrap()
  }

  pub async fn run_script(&mut self, script: SnapshotScript) {
    let rev_manager = self.editor.rev_manager();
    match script {
      SnapshotScript::WriteSnapshot => {
        sleep(Duration::from_millis(2 * REVISION_WRITE_INTERVAL_IN_MILLIS)).await;
        rev_manager.generate_snapshot().await;
        self.current_snapshot = rev_manager.read_snapshot(None).await.unwrap();
      },
      SnapshotScript::AssertSnapshot { rev_id, expected } => {
        let snapshot = rev_manager.read_snapshot(Some(rev_id)).await.unwrap();
        assert_eq!(snapshot, expected);
      },
      SnapshotScript::AssertSnapshotContent { snapshot, expected } => {
        let operations = DatabaseOperations::from_bytes(snapshot.data).unwrap();
        let pad = DatabaseRevisionPad::from_operations(operations).unwrap();
        assert_eq!(pad.json_str().unwrap(), expected);
      },
      SnapshotScript::CreateField { field_rev } => {
        self.editor.create_new_field_rev(field_rev).await.unwrap();
        let current_rev_id = rev_manager.rev_id();
        self.current_revision = rev_manager.get_revision(current_rev_id).await;
      },
      SnapshotScript::DeleteField { field_rev } => {
        self.editor.delete_field(&field_rev.id).await.unwrap();
      },
    }
  }
}
impl std::ops::Deref for DatabaseSnapshotTest {
  type Target = DatabaseEditorTest;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl std::ops::DerefMut for DatabaseSnapshotTest {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
