use std::sync::{Arc, Weak};

use flowy_error::{FlowyError, FlowyResult};
use lib_dispatch::prelude::{data_result_ok, AFPluginData, AFPluginState, DataResult};

use crate::entities::*;
use crate::manager::FolderManager;
use crate::share::ImportParams;

fn upgrade_folder(
  folder_manager: AFPluginState<Weak<FolderManager>>,
) -> FlowyResult<Arc<FolderManager>> {
  let folder = folder_manager
    .upgrade()
    .ok_or(FlowyError::internal().with_context("The folder manager is already dropped"))?;
  Ok(folder)
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn create_workspace_handler(
  data: AFPluginData<CreateWorkspacePayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<WorkspacePB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: CreateWorkspaceParams = data.into_inner().try_into()?;
  let workspace = folder.create_workspace(params).await?;
  let views = folder
    .get_views_belong_to(&workspace.id)
    .await?
    .into_iter()
    .map(view_pb_without_child_views)
    .collect::<Vec<ViewPB>>();
  data_result_ok(WorkspacePB {
    id: workspace.id,
    name: workspace.name,
    views,
    create_time: workspace.created_at,
  })
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn get_workspace_views_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<RepeatedViewPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let child_views = folder.get_current_workspace_views().await?;
  let repeated_view: RepeatedViewPB = child_views.into();
  data_result_ok(repeated_view)
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn open_workspace_handler(
  data: AFPluginData<WorkspaceIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<WorkspacePB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let workspace_id = data.into_inner().value;
  if workspace_id.is_empty() {
    Err(FlowyError::workspace_id().with_context("workspace id should not be empty"))
  } else {
    let workspace = folder.open_workspace(&workspace_id).await?;
    let views = folder.get_workspace_views(&workspace_id).await?;
    let workspace_pb: WorkspacePB = (workspace, views).into();
    data_result_ok(workspace_pb)
  }
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn read_current_workspace_setting_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<WorkspaceSettingPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let setting = folder
    .get_workspace_setting_pb()
    .await
    .ok_or(FlowyError::record_not_found())?;
  data_result_ok(setting)
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn read_current_workspace_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<WorkspacePB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let workspace = folder
    .get_workspace_pb()
    .await
    .ok_or(FlowyError::record_not_found())?;
  data_result_ok(workspace)
}

pub(crate) async fn create_view_handler(
  data: AFPluginData<CreateViewPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<ViewPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: CreateViewParams = data.into_inner().try_into()?;
  let set_as_current = params.set_as_current;
  let view = folder.create_view_with_params(params).await?;
  if set_as_current {
    let _ = folder.set_current_view(&view.id).await;
  }
  data_result_ok(view_pb_without_child_views(Arc::new(view)))
}

pub(crate) async fn create_orphan_view_handler(
  data: AFPluginData<CreateOrphanViewPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<ViewPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: CreateViewParams = data.into_inner().try_into()?;
  let set_as_current = params.set_as_current;
  let view = folder.create_orphan_view_with_params(params).await?;
  if set_as_current {
    let _ = folder.set_current_view(&view.id).await;
  }
  data_result_ok(view_pb_without_child_views(Arc::new(view)))
}

pub(crate) async fn read_view_handler(
  data: AFPluginData<ViewIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<ViewPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let view_id: ViewIdPB = data.into_inner();
  let view_pb = folder.get_view_pb(&view_id.value).await?;
  data_result_ok(view_pb)
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn update_view_handler(
  data: AFPluginData<UpdateViewPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: UpdateViewParams = data.into_inner().try_into()?;
  folder.update_view_with_params(params).await?;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn update_view_icon_handler(
  data: AFPluginData<UpdateViewIconPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: UpdateViewIconParams = data.into_inner().try_into()?;
  folder.update_view_icon_with_params(params).await?;
  Ok(())
}

pub(crate) async fn delete_view_handler(
  data: AFPluginData<RepeatedViewIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: RepeatedViewIdPB = data.into_inner();
  for view_id in &params.items {
    let _ = folder.move_view_to_trash(view_id).await;
  }
  Ok(())
}

pub(crate) async fn toggle_favorites_handler(
  data: AFPluginData<RepeatedViewIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let params: RepeatedViewIdPB = data.into_inner();
  let folder = upgrade_folder(folder)?;
  for view_id in &params.items {
    let _ = folder.toggle_favorites(view_id).await;
  }
  Ok(())
}

pub(crate) async fn set_latest_view_handler(
  data: AFPluginData<ViewIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let view_id: ViewIdPB = data.into_inner();
  let _ = folder.set_current_view(&view_id.value).await;
  Ok(())
}

pub(crate) async fn close_view_handler(
  data: AFPluginData<ViewIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let view_id: ViewIdPB = data.into_inner();
  let _ = folder.close_view(&view_id.value).await;
  Ok(())
}

#[tracing::instrument(level = "debug", skip_all, err)]
pub(crate) async fn move_view_handler(
  data: AFPluginData<MoveViewPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: MoveViewParams = data.into_inner().try_into()?;
  folder
    .move_view(&params.view_id, params.from, params.to)
    .await?;
  Ok(())
}

pub(crate) async fn move_nested_view_handler(
  data: AFPluginData<MoveNestedViewPayloadPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: MoveNestedViewParams = data.into_inner().try_into()?;
  folder
    .move_nested_view(params.view_id, params.new_parent_id, params.prev_view_id)
    .await?;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn duplicate_view_handler(
  data: AFPluginData<ViewPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let view: ViewPB = data.into_inner();
  folder.duplicate_view(&view.id).await?;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn read_favorites_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<RepeatedViewPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let favorite_items = folder.get_all_favorites().await;
  let mut views = vec![];
  for item in favorite_items {
    if let Ok(view) = folder.get_view_pb(&item.id).await {
      views.push(view);
    }
  }
  data_result_ok(RepeatedViewPB { items: views })
}
#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn read_trash_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<RepeatedTrashPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let trash = folder.get_all_trash().await;
  data_result_ok(trash.into())
}

#[tracing::instrument(level = "debug", skip(identifier, folder), err)]
pub(crate) async fn putback_trash_handler(
  identifier: AFPluginData<TrashIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  folder.restore_trash(&identifier.id).await;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(identifiers, folder), err)]
pub(crate) async fn delete_trash_handler(
  identifiers: AFPluginData<RepeatedTrashIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let trash_ids = identifiers.into_inner().items;
  for trash_id in trash_ids {
    let _ = folder.delete_trash(&trash_id.id).await;
  }
  Ok(())
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn restore_all_trash_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  folder.restore_all_trash().await;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn delete_all_trash_handler(
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  folder.delete_all_trash().await;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(data, folder), err)]
pub(crate) async fn import_data_handler(
  data: AFPluginData<ImportPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> Result<(), FlowyError> {
  let folder = upgrade_folder(folder)?;
  let params: ImportParams = data.into_inner().try_into()?;
  folder.import(params).await?;
  Ok(())
}

#[tracing::instrument(level = "debug", skip(folder), err)]
pub(crate) async fn get_folder_snapshots_handler(
  data: AFPluginData<WorkspaceIdPB>,
  folder: AFPluginState<Weak<FolderManager>>,
) -> DataResult<RepeatedFolderSnapshotPB, FlowyError> {
  let folder = upgrade_folder(folder)?;
  let data = data.into_inner();
  let snapshots = folder.get_folder_snapshots(&data.value, 10).await?;
  data_result_ok(RepeatedFolderSnapshotPB { items: snapshots })
}
