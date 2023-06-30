// Task related models

use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::tools::duration_to_string;

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum TaskType {
    EncodeOriginal = 0,
    EncodeResolution = 1,
    GenerateVideoPreviews = 2,
}

impl TaskType {
    pub fn to_string(&self) -> String {
        match self {
            TaskType::EncodeOriginal => {
                return "Encode original".to_string();
            }
            TaskType::EncodeResolution => {
                return "Encode resolution".to_string();
            }
            TaskType::GenerateVideoPreviews => {
                return "Generate video previews".to_string();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskEncodeResolution {
    #[serde(rename = "width")]
    pub width: i32,

    #[serde(rename = "height")]
    pub height: i32,

    #[serde(rename = "fps")]
    pub fps: i32,
}

impl TaskEncodeResolution {
    pub fn to_string(&self) -> String {
        let w = self.width;
        let h = self.height;
        let fps = self.fps;
        if fps > 0 {
            return format!("{w}x{h}:{fps}");
        } else {
            return format!("{w}x{h}");
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStage {
    #[serde(rename = "")]
    Pending,

    #[serde(rename = "PREPARE")]
    Prepare,

    #[serde(rename = "COPY")]
    Copy,

    #[serde(rename = "PROBE")]
    Probe,

    #[serde(rename = "ENCODE")]
    Encode,

    #[serde(rename = "ENCRYPT")]
    Encrypt,

    #[serde(rename = "UPDATE")]
    Update,

    #[serde(rename = "FINISH")]
    Finish,
}

impl TaskStage {
    pub fn to_string(&self) -> String {
        match self {
            TaskStage::Pending => {
                return "Pending".to_string();
            }
            TaskStage::Prepare => {
                return "Prepare".to_string();
            }
            TaskStage::Copy => {
                return "Copy".to_string();
            }
            TaskStage::Probe => {
                return "Probe".to_string();
            }
            TaskStage::Encode => {
                return "Encode".to_string();
            }
            TaskStage::Encrypt => {
                return "Encrypt".to_string();
            }
            TaskStage::Update => {
                return "Update".to_string();
            }
            TaskStage::Finish => {
                return "Finish".to_string();
            }
        }
    }

    pub fn to_stage_number(&self) -> u8 {
        match self {
            TaskStage::Pending => 0,
            TaskStage::Prepare => 0,
            TaskStage::Copy => 1,
            TaskStage::Probe => 2,
            TaskStage::Encode => 3,
            TaskStage::Encrypt => 4,
            TaskStage::Update => 5,
            TaskStage::Finish => 6,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "id")]
    pub id: u64,

    #[serde(rename = "running")]
    pub running: bool,

    #[serde(rename = "media_id")]
    pub media_id: u64,

    #[serde(rename = "type")]
    pub task_type: TaskType,

    #[serde(rename = "resolution")]
    pub resolution: Option<TaskEncodeResolution>,

    #[serde(rename = "stage")]
    pub stage: TaskStage,

    #[serde(rename = "stage_start")]
    pub stage_start: i64,

    #[serde(rename = "time_now")]
    pub time_now: i64,

    #[serde(rename = "stage_progress")]
    pub stage_progress: f64,
}

pub fn get_task_type_string(task: &Task) -> String {
    match task.task_type {
        TaskType::EncodeOriginal | TaskType::GenerateVideoPreviews => {
            return task.task_type.to_string();
        }
        TaskType::EncodeResolution => {
            match &task.resolution {
                Some(r) => {
                    let task_type = task.task_type.to_string();
                    let resolution = r.to_string();
                    return format!("{task_type}: {resolution}")
                }
                None => {
                    return task.task_type.to_string();
                }
            }
        }
    }
}

pub fn get_task_status_string(task: &Task) -> String {
    if task.running {
        match task.stage {
            TaskStage::Pending => {
                return "Pending".to_string();
            }
            TaskStage::Prepare
            | TaskStage::Copy
            | TaskStage::Probe
            | TaskStage::Encode
            | TaskStage::Encrypt
            | TaskStage::Update
            | TaskStage::Finish => {
                let stage_number = task.stage.to_stage_number() + 1;
                let stage_name = task.stage.to_string();

                if task.stage_progress > 0.0 {
                    let p = task.stage_progress;
                    return format!("Stage {stage_number}/7: {stage_name} ({p:.2}%)");
                } else {
                    return format!("Stage {stage_number}/7: {stage_name}");
                }
            }
        }
    } else {
        return "Pending".to_string();
    }
}

pub fn get_task_remaining_time_string(task: &Task) -> String {
    if !task.running {
        return "N/A".to_string();
    }

    if task.stage_progress <= 0.0 {
        return "Unknown".to_string();
    }

    let p = task.stage_progress;
    let now = task.time_now;
    let start = task.stage_start;

    if now <= start {
        return "Unknown".to_string();
    }

    let task_time = (now - start) as f64;

    let estimated_remaining_time_seconds = ((task_time * 100.0 / p) - task_time) / 1000.0;

    return duration_to_string(estimated_remaining_time_seconds);
}
