//! 由 `xtask convert-config` 生成，**不要手改**。
//!
//! 字段名沿用旧仓的 CamelCase（`rename`），这样老配置文件可以直接读。
//! 重新生成：`cargo run -p xtask -- convert-config`

#![allow(missing_docs)]

use serde::Deserialize;

/// `Alas.DailySummary` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasDailySummary {
    /// 旧键 `Alas.DailySummary.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Alas.DailySummary.TriggerTime`
    #[serde(rename = "TriggerTime", default)]
    pub trigger_time: Option<String>,
}

/// `Alas.DropRecord` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasDropRecord {
    /// 旧键 `Alas.DropRecord.API`
    #[serde(rename = "API", default)]
    pub api: Option<String>,
    /// 旧键 `Alas.DropRecord.BugReport`
    #[serde(rename = "BugReport", default)]
    pub bug_report: Option<bool>,
    /// 旧键 `Alas.DropRecord.OpsiRecord`
    #[serde(rename = "OpsiRecord", default)]
    pub opsi_record: Option<String>,
    /// 旧键 `Alas.DropRecord.SaveFolder`
    #[serde(rename = "SaveFolder", default)]
    pub save_folder: Option<String>,
    /// 旧键 `Alas.DropRecord.AzurStatsID`
    #[serde(rename = "AzurStatsID", default)]
    pub azur_stats_id: Option<String>,
    /// 旧键 `Alas.DropRecord.CombatRecord`
    #[serde(rename = "CombatRecord", default)]
    pub combat_record: Option<String>,
    /// 旧键 `Alas.DropRecord.MeowfficerBuy`
    #[serde(rename = "MeowfficerBuy", default)]
    pub meowfficer_buy: Option<String>,
    /// 旧键 `Alas.DropRecord.ResearchRecord`
    #[serde(rename = "ResearchRecord", default)]
    pub research_record: Option<String>,
    /// 旧键 `Alas.DropRecord.TelemetryReport`
    #[serde(rename = "TelemetryReport", default)]
    pub telemetry_report: Option<bool>,
    /// 旧键 `Alas.DropRecord.CommissionRecord`
    #[serde(rename = "CommissionRecord", default)]
    pub commission_record: Option<String>,
    /// 旧键 `Alas.DropRecord.MeowfficerTalent`
    #[serde(rename = "MeowfficerTalent", default)]
    pub meowfficer_talent: Option<String>,
}

/// `Alas.Emulator` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasEmulator {
    /// 旧键 `Alas.Emulator.Serial`
    #[serde(rename = "Serial", default)]
    pub serial: Option<String>,
    /// 旧键 `Alas.Emulator.AdbRestart`
    #[serde(rename = "AdbRestart", default)]
    pub adb_restart: Option<bool>,
    /// 旧键 `Alas.Emulator.ServerName`
    #[serde(rename = "ServerName", default)]
    pub server_name: Option<String>,
    /// 旧键 `Alas.Emulator.PackageName`
    #[serde(rename = "PackageName", default)]
    pub package_name: Option<String>,
    /// 旧键 `Alas.Emulator.GameSettings`
    #[serde(rename = "GameSettings", default)]
    pub game_settings: Option<bool>,
    /// 旧键 `Alas.Emulator.ControlMethod`
    #[serde(rename = "ControlMethod", default)]
    pub control_method: Option<String>,
    /// 旧键 `Alas.Emulator.ScreenshotMethod`
    #[serde(rename = "ScreenshotMethod", default)]
    pub screenshot_method: Option<String>,
    /// 旧键 `Alas.Emulator.ScreenshotDedithering`
    #[serde(rename = "ScreenshotDedithering", default)]
    pub screenshot_dedithering: Option<bool>,
}

/// `Alas.EmulatorInfo` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasEmulatorInfo {
    /// 旧键 `Alas.EmulatorInfo.name`
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.path`
    #[serde(rename = "path", default)]
    pub path: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.Emulator`
    #[serde(rename = "Emulator", default)]
    pub emulator: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.RemoteSSHHost`
    #[serde(rename = "RemoteSSHHost", default)]
    pub remote_sshhost: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.RemoteSSHPort`
    #[serde(rename = "RemoteSSHPort", default)]
    pub remote_sshport: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.RemoteSSHUser`
    #[serde(rename = "RemoteSSHUser", default)]
    pub remote_sshuser: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.EnableRemoteSSH`
    #[serde(rename = "EnableRemoteSSH", default)]
    pub enable_remote_ssh: Option<bool>,
    /// 旧键 `Alas.EmulatorInfo.RemoteStopCommand`
    #[serde(rename = "RemoteStopCommand", default)]
    pub remote_stop_command: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.RemoteSSHPublicKey`
    #[serde(rename = "RemoteSSHPublicKey", default)]
    pub remote_sshpublic_key: Option<String>,
    /// 旧键 `Alas.EmulatorInfo.RemoteStartCommand`
    #[serde(rename = "RemoteStartCommand", default)]
    pub remote_start_command: Option<String>,
}

/// `Alas.EmulatorManagement` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasEmulatorManagement {
    /// 旧键 `Alas.EmulatorManagement.ForceScheduledRestart`
    #[serde(rename = "ForceScheduledRestart", default)]
    pub force_scheduled_restart: Option<bool>,
    /// 旧键 `Alas.EmulatorManagement.ScheduledEmulatorRestart`
    #[serde(rename = "ScheduledEmulatorRestart", default)]
    pub scheduled_emulator_restart: Option<bool>,
    /// 旧键 `Alas.EmulatorManagement.RestartIntervalHours`
    #[serde(rename = "RestartIntervalHours", default)]
    pub restart_interval_hours: Option<String>,
}

/// `Alas.Error` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasError {
    /// 旧键 `Alas.Error.LlmModel`
    #[serde(rename = "LlmModel", default)]
    pub llm_model: Option<String>,
    /// 旧键 `Alas.Error.LlmApiKey`
    #[serde(rename = "LlmApiKey", default)]
    pub llm_api_key: Option<String>,
    /// 旧键 `Alas.Error.SaveError`
    #[serde(rename = "SaveError", default)]
    pub save_error: Option<bool>,
    /// 旧键 `Alas.Error.LlmApiBase`
    #[serde(rename = "LlmApiBase", default)]
    pub llm_api_base: Option<String>,
    /// 旧键 `Alas.Error.HandleError`
    #[serde(rename = "HandleError", default)]
    pub handle_error: Option<bool>,
    /// 旧键 `Alas.Error.LlmAnalysis`
    #[serde(rename = "LlmAnalysis", default)]
    pub llm_analysis: Option<bool>,
    /// 旧键 `Alas.Error.OnePushConfig`
    #[serde(rename = "OnePushConfig", default)]
    pub one_push_config: Option<String>,
    /// 旧键 `Alas.Error.StrictRestart`
    #[serde(rename = "StrictRestart", default)]
    pub strict_restart: Option<bool>,
    /// 旧键 `Alas.Error.SaveErrorCount`
    #[serde(rename = "SaveErrorCount", default)]
    pub save_error_count: Option<String>,
    /// 旧键 `Alas.Error.WatchdogEnable`
    #[serde(rename = "WatchdogEnable", default)]
    pub watchdog_enable: Option<bool>,
    /// 旧键 `Alas.Error.GameStuckRestart`
    #[serde(rename = "GameStuckRestart", default)]
    pub game_stuck_restart: Option<bool>,
    /// 旧键 `Alas.Error.ScreenshotLength`
    #[serde(rename = "ScreenshotLength", default)]
    pub screenshot_length: Option<String>,
    /// 旧键 `Alas.Error.AdbOfflineRestart`
    #[serde(rename = "AdbOfflineRestart", default)]
    pub adb_offline_restart: Option<bool>,
    /// 旧键 `Alas.Error.GameStuckThreshold`
    #[serde(rename = "GameStuckThreshold", default)]
    pub game_stuck_threshold: Option<String>,
    /// 旧键 `Alas.Error.WatchdogTaskEnable`
    #[serde(rename = "WatchdogTaskEnable", default)]
    pub watchdog_task_enable: Option<bool>,
    /// 旧键 `Alas.Error.AdbOfflineThreshold`
    #[serde(rename = "AdbOfflineThreshold", default)]
    pub adb_offline_threshold: Option<String>,
    /// 旧键 `Alas.Error.WatchdogTaskTimeout`
    #[serde(rename = "WatchdogTaskTimeout", default)]
    pub watchdog_task_timeout: Option<String>,
    /// 旧键 `Alas.Error.RestartOperationTimeout`
    #[serde(rename = "RestartOperationTimeout", default)]
    pub restart_operation_timeout: Option<String>,
    /// 旧键 `Alas.Error.RestartOperationTimeoutEnable`
    #[serde(rename = "RestartOperationTimeoutEnable", default)]
    pub restart_operation_timeout_enable: Option<bool>,
}

/// `Alas.Optimization` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AlasOptimization {
    /// 旧键 `Alas.Optimization.OcrDevice`
    #[serde(rename = "OcrDevice", default)]
    pub ocr_device: Option<String>,
    /// 旧键 `Alas.Optimization.OcrBackend`
    #[serde(rename = "OcrBackend", default)]
    pub ocr_backend: Option<String>,
    /// 旧键 `Alas.Optimization.ScreenshotInterval`
    #[serde(rename = "ScreenshotInterval", default)]
    pub screenshot_interval: Option<String>,
    /// 旧键 `Alas.Optimization.WhenTaskQueueEmpty`
    #[serde(rename = "WhenTaskQueueEmpty", default)]
    pub when_task_queue_empty: Option<String>,
    /// 旧键 `Alas.Optimization.OcrWindowsMlVendorEp`
    #[serde(rename = "OcrWindowsMlVendorEp", default)]
    pub ocr_windows_ml_vendor_ep: Option<bool>,
    /// 旧键 `Alas.Optimization.TaskHoardingDuration`
    #[serde(rename = "TaskHoardingDuration", default)]
    pub task_hoarding_duration: Option<String>,
    /// 旧键 `Alas.Optimization.WhenSchedulerStopped`
    #[serde(rename = "WhenSchedulerStopped", default)]
    pub when_scheduler_stopped: Option<String>,
    /// 旧键 `Alas.Optimization.OcrModelVersionChinese`
    #[serde(rename = "OcrModelVersionChinese", default)]
    pub ocr_model_version_chinese: Option<String>,
    /// 旧键 `Alas.Optimization.OcrModelVersionEnglish`
    #[serde(rename = "OcrModelVersionEnglish", default)]
    pub ocr_model_version_english: Option<String>,
    /// 旧键 `Alas.Optimization.OcrModelVersionJapanese`
    #[serde(rename = "OcrModelVersionJapanese", default)]
    pub ocr_model_version_japanese: Option<String>,
    /// 旧键 `Alas.Optimization.CombatScreenshotInterval`
    #[serde(rename = "CombatScreenshotInterval", default)]
    pub combat_screenshot_interval: Option<String>,
    /// 旧键 `Alas.Optimization.CloseEmulatorDuringLongWait`
    #[serde(rename = "CloseEmulatorDuringLongWait", default)]
    pub close_emulator_during_long_wait: Option<bool>,
    /// 旧键 `Alas.Optimization.OcrModelVersionTraditionalChinese`
    #[serde(rename = "OcrModelVersionTraditionalChinese", default)]
    pub ocr_model_version_traditional_chinese: Option<String>,
}

/// `Alas` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Alas {
    /// 旧键 `Alas.DailySummary.*`
    #[serde(rename = "DailySummary", default)]
    pub daily_summary: Option<AlasDailySummary>,
    /// 旧键 `Alas.DropRecord.*`
    #[serde(rename = "DropRecord", default)]
    pub drop_record: Option<AlasDropRecord>,
    /// 旧键 `Alas.Emulator.*`
    #[serde(rename = "Emulator", default)]
    pub emulator: Option<AlasEmulator>,
    /// 旧键 `Alas.EmulatorInfo.*`
    #[serde(rename = "EmulatorInfo", default)]
    pub emulator_info: Option<AlasEmulatorInfo>,
    /// 旧键 `Alas.EmulatorManagement.*`
    #[serde(rename = "EmulatorManagement", default)]
    pub emulator_management: Option<AlasEmulatorManagement>,
    /// 旧键 `Alas.Error.*`
    #[serde(rename = "Error", default)]
    pub error: Option<AlasError>,
    /// 旧键 `Alas.Optimization.*`
    #[serde(rename = "Optimization", default)]
    pub optimization: Option<AlasOptimization>,
}

/// `Ambush11.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11Campaign {
    /// 旧键 `Ambush11.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Ambush11.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Ambush11.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Ambush11.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11Emotion {
    /// 旧键 `Ambush11.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Ambush11.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Ambush11.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Ambush11.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Ambush11.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Ambush11.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Ambush11.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11Fleet {
    /// 旧键 `Ambush11.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Ambush11.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Ambush11.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Ambush11.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Ambush11.GemsFarming` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11GemsFarming {
    /// 旧键 `Ambush11.GemsFarming.CommonCV`
    #[serde(rename = "CommonCV", default)]
    pub common_cv: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.CommonDD`
    #[serde(rename = "CommonDD", default)]
    pub common_dd: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.EquipmentCode`
    #[serde(rename = "EquipmentCode", default)]
    pub equipment_code: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.ChangeFlagship`
    #[serde(rename = "ChangeFlagship", default)]
    pub change_flagship: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.ChangeVanguard`
    #[serde(rename = "ChangeVanguard", default)]
    pub change_vanguard: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.CommonCVFilter`
    #[serde(rename = "CommonCVFilter", default)]
    pub common_cvfilter: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.CommonDDFilter`
    #[serde(rename = "CommonDDFilter", default)]
    pub common_ddfilter: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.CommissionLimit`
    #[serde(rename = "CommissionLimit", default)]
    pub commission_limit: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.UseEmotionFirst`
    #[serde(rename = "UseEmotionFirst", default)]
    pub use_emotion_first: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.IgnoreEmotionWarning`
    #[serde(rename = "IgnoreEmotionWarning", default)]
    pub ignore_emotion_warning: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.AllowLowVanguardLevel`
    #[serde(rename = "AllowLowVanguardLevel", default)]
    pub allow_low_vanguard_level: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.DelayTaskIFNoFlagship`
    #[serde(rename = "DelayTaskIFNoFlagship", default)]
    pub delay_task_ifno_flagship: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.AllowHighFlagshipLevel`
    #[serde(rename = "AllowHighFlagshipLevel", default)]
    pub allow_high_flagship_level: Option<bool>,
    /// 旧键 `Ambush11.GemsFarming.HighValueCommissionReserve`
    #[serde(rename = "HighValueCommissionReserve", default)]
    pub high_value_commission_reserve: Option<String>,
    /// 旧键 `Ambush11.GemsFarming.HighValueCommissionFilterCount`
    #[serde(rename = "HighValueCommissionFilterCount", default)]
    pub high_value_commission_filter_count: Option<String>,
}

/// `Ambush11.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11Scheduler {
    /// 旧键 `Ambush11.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Ambush11.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Ambush11.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Ambush11.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Ambush11.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11StopCondition {
    /// 旧键 `Ambush11.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Ambush11.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `Ambush11.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11Submarine {
    /// 旧键 `Ambush11.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Ambush11.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `Ambush11` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Ambush11 {
    /// 旧键 `Ambush11.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<Ambush11Campaign>,
    /// 旧键 `Ambush11.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<Ambush11Emotion>,
    /// 旧键 `Ambush11.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<Ambush11Fleet>,
    /// 旧键 `Ambush11.GemsFarming.*`
    #[serde(rename = "GemsFarming", default)]
    pub gems_farming: Option<Ambush11GemsFarming>,
    /// 旧键 `Ambush11.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<Ambush11Scheduler>,
    /// 旧键 `Ambush11.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<Ambush11StopCondition>,
    /// 旧键 `Ambush11.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<Ambush11Submarine>,
}

/// `AutoEquip.AutoEquip` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AutoEquipAutoEquip {
    /// 旧键 `AutoEquip.AutoEquip.ShipLimit`
    #[serde(rename = "ShipLimit", default)]
    pub ship_limit: Option<String>,
    /// 旧键 `AutoEquip.AutoEquip.EnableSlot1`
    #[serde(rename = "EnableSlot1", default)]
    pub enable_slot1: Option<bool>,
    /// 旧键 `AutoEquip.AutoEquip.EnableSlot2`
    #[serde(rename = "EnableSlot2", default)]
    pub enable_slot2: Option<bool>,
    /// 旧键 `AutoEquip.AutoEquip.EnableSlot3`
    #[serde(rename = "EnableSlot3", default)]
    pub enable_slot3: Option<bool>,
    /// 旧键 `AutoEquip.AutoEquip.EnableSlot4`
    #[serde(rename = "EnableSlot4", default)]
    pub enable_slot4: Option<bool>,
    /// 旧键 `AutoEquip.AutoEquip.EnableSlot5`
    #[serde(rename = "EnableSlot5", default)]
    pub enable_slot5: Option<bool>,
}

/// `AutoEquip` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AutoEquip {
    /// 旧键 `AutoEquip.AutoEquip.*`
    #[serde(rename = "AutoEquip", default)]
    pub auto_equip: Option<AutoEquipAutoEquip>,
}

/// `Awaken.Awaken` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AwakenAwaken {
    /// 旧键 `Awaken.Awaken.LevelCap`
    #[serde(rename = "LevelCap", default)]
    pub level_cap: Option<String>,
    /// 旧键 `Awaken.Awaken.Favourite`
    #[serde(rename = "Favourite", default)]
    pub favourite: Option<bool>,
}

/// `Awaken.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AwakenScheduler {
    /// 旧键 `Awaken.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Awaken.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Awaken.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Awaken.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Awaken` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Awaken {
    /// 旧键 `Awaken.Awaken.*`
    #[serde(rename = "Awaken", default)]
    pub awaken: Option<AwakenAwaken>,
    /// 旧键 `Awaken.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<AwakenScheduler>,
}

/// `AzurLaneUncensored.AzurLaneUncensored` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AzurLaneUncensoredAzurLaneUncensored {
    /// 旧键 `AzurLaneUncensored.AzurLaneUncensored.Repository`
    #[serde(rename = "Repository", default)]
    pub repository: Option<String>,
}

/// `AzurLaneUncensored` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct AzurLaneUncensored {
    /// 旧键 `AzurLaneUncensored.AzurLaneUncensored.*`
    #[serde(rename = "AzurLaneUncensored", default)]
    pub azur_lane_uncensored: Option<AzurLaneUncensoredAzurLaneUncensored>,
}

/// `Benchmark.Benchmark` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct BenchmarkBenchmark {
    /// 旧键 `Benchmark.Benchmark.TestScene`
    #[serde(rename = "TestScene", default)]
    pub test_scene: Option<String>,
    /// 旧键 `Benchmark.Benchmark.DeviceType`
    #[serde(rename = "DeviceType", default)]
    pub device_type: Option<String>,
}

/// `Benchmark` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Benchmark {
    /// 旧键 `Benchmark.Benchmark.*`
    #[serde(rename = "Benchmark", default)]
    pub benchmark: Option<BenchmarkBenchmark>,
}

/// `BoxDisassemble.BoxDisassemble` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct BoxDisassembleBoxDisassemble {
    /// 旧键 `BoxDisassemble.BoxDisassemble.UseBlueBox`
    #[serde(rename = "UseBlueBox", default)]
    pub use_blue_box: Option<bool>,
    /// 旧键 `BoxDisassemble.BoxDisassemble.UseWhiteBox`
    #[serde(rename = "UseWhiteBox", default)]
    pub use_white_box: Option<bool>,
    /// 旧键 `BoxDisassemble.BoxDisassemble.BlueBoxLimit`
    #[serde(rename = "BlueBoxLimit", default)]
    pub blue_box_limit: Option<String>,
    /// 旧键 `BoxDisassemble.BoxDisassemble.UsePurpleBox`
    #[serde(rename = "UsePurpleBox", default)]
    pub use_purple_box: Option<bool>,
    /// 旧键 `BoxDisassemble.BoxDisassemble.WhiteBoxLimit`
    #[serde(rename = "WhiteBoxLimit", default)]
    pub white_box_limit: Option<String>,
    /// 旧键 `BoxDisassemble.BoxDisassemble.PurpleBoxLimit`
    #[serde(rename = "PurpleBoxLimit", default)]
    pub purple_box_limit: Option<String>,
}

/// `BoxDisassemble` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct BoxDisassemble {
    /// 旧键 `BoxDisassemble.BoxDisassemble.*`
    #[serde(rename = "BoxDisassemble", default)]
    pub box_disassemble: Option<BoxDisassembleBoxDisassemble>,
}

/// `Coalition.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionCampaign {
    /// 旧键 `Coalition.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Coalition.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Coalition.Coalition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionCoalition {
    /// 旧键 `Coalition.Coalition.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Coalition.Coalition.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `Coalition.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionEmotion {
    /// 旧键 `Coalition.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Coalition.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Coalition.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Coalition.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Coalition.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Coalition.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Coalition.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Coalition.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
}

/// `Coalition.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScheduler {
    /// 旧键 `Coalition.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Coalition.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Coalition.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Coalition.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Coalition.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionStopCondition {
    /// 旧键 `Coalition.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Coalition.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Coalition.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `Coalition` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Coalition {
    /// 旧键 `Coalition.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<CoalitionCampaign>,
    /// 旧键 `Coalition.Coalition.*`
    #[serde(rename = "Coalition", default)]
    pub coalition: Option<CoalitionCoalition>,
    /// 旧键 `Coalition.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<CoalitionEmotion>,
    /// 旧键 `Coalition.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<CoalitionScheduler>,
    /// 旧键 `Coalition.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<CoalitionStopCondition>,
}

/// `CoalitionScuttle.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleCampaign {
    /// 旧键 `CoalitionScuttle.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `CoalitionScuttle.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `CoalitionScuttle.Coalition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleCoalition {
    /// 旧键 `CoalitionScuttle.Coalition.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `CoalitionScuttle.Coalition.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `CoalitionScuttle.CoalitionScuttle` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleCoalitionScuttle {
    /// 旧键 `CoalitionScuttle.CoalitionScuttle.Sacrifice`
    #[serde(rename = "Sacrifice", default)]
    pub sacrifice: Option<String>,
}

/// `CoalitionScuttle.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleEmotion {
    /// 旧键 `CoalitionScuttle.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `CoalitionScuttle.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `CoalitionScuttle.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleScheduler {
    /// 旧键 `CoalitionScuttle.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `CoalitionScuttle.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `CoalitionScuttle.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `CoalitionScuttle.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `CoalitionScuttle.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttleStopCondition {
    /// 旧键 `CoalitionScuttle.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `CoalitionScuttle.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `CoalitionScuttle.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `CoalitionScuttle` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionScuttle {
    /// 旧键 `CoalitionScuttle.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<CoalitionScuttleCampaign>,
    /// 旧键 `CoalitionScuttle.Coalition.*`
    #[serde(rename = "Coalition", default)]
    pub coalition: Option<CoalitionScuttleCoalition>,
    /// 旧键 `CoalitionScuttle.CoalitionScuttle.*`
    #[serde(rename = "CoalitionScuttle", default)]
    pub coalition_scuttle: Option<CoalitionScuttleCoalitionScuttle>,
    /// 旧键 `CoalitionScuttle.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<CoalitionScuttleEmotion>,
    /// 旧键 `CoalitionScuttle.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<CoalitionScuttleScheduler>,
    /// 旧键 `CoalitionScuttle.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<CoalitionScuttleStopCondition>,
}

/// `CoalitionSp.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSpCampaign {
    /// 旧键 `CoalitionSp.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `CoalitionSp.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `CoalitionSp.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `CoalitionSp.Coalition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSpCoalition {
    /// 旧键 `CoalitionSp.Coalition.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `CoalitionSp.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSpEmotion {
    /// 旧键 `CoalitionSp.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `CoalitionSp.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `CoalitionSp.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
}

/// `CoalitionSp.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSpScheduler {
    /// 旧键 `CoalitionSp.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `CoalitionSp.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `CoalitionSp.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `CoalitionSp.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `CoalitionSp.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSpStopCondition {
    /// 旧键 `CoalitionSp.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `CoalitionSp.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `CoalitionSp.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `CoalitionSp` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CoalitionSp {
    /// 旧键 `CoalitionSp.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<CoalitionSpCampaign>,
    /// 旧键 `CoalitionSp.Coalition.*`
    #[serde(rename = "Coalition", default)]
    pub coalition: Option<CoalitionSpCoalition>,
    /// 旧键 `CoalitionSp.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<CoalitionSpEmotion>,
    /// 旧键 `CoalitionSp.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<CoalitionSpScheduler>,
    /// 旧键 `CoalitionSp.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<CoalitionSpStopCondition>,
}

/// `Commission.Commission` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CommissionCommission {
    /// 旧键 `Commission.Commission.Blacklist`
    #[serde(rename = "Blacklist", default)]
    pub blacklist: Option<String>,
    /// 旧键 `Commission.Commission.CustomFilter`
    #[serde(rename = "CustomFilter", default)]
    pub custom_filter: Option<String>,
    /// 旧键 `Commission.Commission.PresetFilter`
    #[serde(rename = "PresetFilter", default)]
    pub preset_filter: Option<String>,
    /// 旧键 `Commission.Commission.DelayHalfLife`
    #[serde(rename = "DelayHalfLife", default)]
    pub delay_half_life: Option<String>,
    /// 旧键 `Commission.Commission.TierValueRatio`
    #[serde(rename = "TierValueRatio", default)]
    pub tier_value_ratio: Option<String>,
    /// 旧键 `Commission.Commission.FilterValueFloor`
    #[serde(rename = "FilterValueFloor", default)]
    pub filter_value_floor: Option<String>,
    /// 旧键 `Commission.Commission.DoMajorCommission`
    #[serde(rename = "DoMajorCommission", default)]
    pub do_major_commission: Option<bool>,
    /// 旧键 `Commission.Commission.DynamicProgramming`
    #[serde(rename = "DynamicProgramming", default)]
    pub dynamic_programming: Option<bool>,
    /// 旧键 `Commission.Commission.FilterValueHalfLife`
    #[serde(rename = "FilterValueHalfLife", default)]
    pub filter_value_half_life: Option<String>,
    /// 旧键 `Commission.Commission.DeadlineFutureHorizon`
    #[serde(rename = "DeadlineFutureHorizon", default)]
    pub deadline_future_horizon: Option<String>,
    /// 旧键 `Commission.Commission.CommissionNotifyReward`
    #[serde(rename = "CommissionNotifyReward", default)]
    pub commission_notify_reward: Option<bool>,
    /// 旧键 `Commission.Commission.CommissionNotifyRewardStatistics`
    #[serde(rename = "CommissionNotifyRewardStatistics", default)]
    pub commission_notify_reward_statistics: Option<bool>,
}

/// `Commission.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CommissionScheduler {
    /// 旧键 `Commission.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Commission.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Commission.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Commission.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Commission` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Commission {
    /// 旧键 `Commission.Commission.*`
    #[serde(rename = "Commission", default)]
    pub commission: Option<CommissionCommission>,
    /// 旧键 `Commission.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<CommissionScheduler>,
}

/// `Daemon.Daemon` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DaemonDaemon {
    /// 旧键 `Daemon.Daemon.EnterMap`
    #[serde(rename = "EnterMap", default)]
    pub enter_map: Option<bool>,
}

/// `Daemon` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Daemon {
    /// 旧键 `Daemon.Daemon.*`
    #[serde(rename = "Daemon", default)]
    pub daemon: Option<DaemonDaemon>,
}

/// `Daily.Daily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DailyDaily {
    /// 旧键 `Daily.Daily.UseDailySkip`
    #[serde(rename = "UseDailySkip", default)]
    pub use_daily_skip: Option<bool>,
    /// 旧键 `Daily.Daily.EscortMission`
    #[serde(rename = "EscortMission", default)]
    pub escort_mission: Option<String>,
    /// 旧键 `Daily.Daily.FierceAssault`
    #[serde(rename = "FierceAssault", default)]
    pub fierce_assault: Option<String>,
    /// 旧键 `Daily.Daily.AdvanceMission`
    #[serde(rename = "AdvanceMission", default)]
    pub advance_mission: Option<String>,
    /// 旧键 `Daily.Daily.TacticalTraining`
    #[serde(rename = "TacticalTraining", default)]
    pub tactical_training: Option<String>,
    /// 旧键 `Daily.Daily.ModuleDevelopment`
    #[serde(rename = "ModuleDevelopment", default)]
    pub module_development: Option<String>,
    /// 旧键 `Daily.Daily.EscortMissionFleet`
    #[serde(rename = "EscortMissionFleet", default)]
    pub escort_mission_fleet: Option<String>,
    /// 旧键 `Daily.Daily.FierceAssaultFleet`
    #[serde(rename = "FierceAssaultFleet", default)]
    pub fierce_assault_fleet: Option<String>,
    /// 旧键 `Daily.Daily.AdvanceMissionFleet`
    #[serde(rename = "AdvanceMissionFleet", default)]
    pub advance_mission_fleet: Option<String>,
    /// 旧键 `Daily.Daily.SupplyLineDisruption`
    #[serde(rename = "SupplyLineDisruption", default)]
    pub supply_line_disruption: Option<String>,
    /// 旧键 `Daily.Daily.TacticalTrainingFleet`
    #[serde(rename = "TacticalTrainingFleet", default)]
    pub tactical_training_fleet: Option<String>,
    /// 旧键 `Daily.Daily.ModuleDevelopmentFleet`
    #[serde(rename = "ModuleDevelopmentFleet", default)]
    pub module_development_fleet: Option<String>,
    /// 旧键 `Daily.Daily.EmergencyModuleDevelopment`
    #[serde(rename = "EmergencyModuleDevelopment", default)]
    pub emergency_module_development: Option<String>,
    /// 旧键 `Daily.Daily.EmergencyModuleDevelopmentFleet`
    #[serde(rename = "EmergencyModuleDevelopmentFleet", default)]
    pub emergency_module_development_fleet: Option<String>,
}

/// `Daily.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DailyScheduler {
    /// 旧键 `Daily.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Daily.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Daily.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Daily.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Daily` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Daily {
    /// 旧键 `Daily.Daily.*`
    #[serde(rename = "Daily", default)]
    pub daily: Option<DailyDaily>,
    /// 旧键 `Daily.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<DailyScheduler>,
}

/// `Dashboard.ActionPoint` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardActionPoint {
    /// 旧键 `Dashboard.ActionPoint.Total`
    #[serde(rename = "Total", default)]
    pub total: Option<String>,
    /// 旧键 `Dashboard.ActionPoint.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.ActionPoint.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Coin` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardCoin {
    /// 旧键 `Dashboard.Coin.Limit`
    #[serde(rename = "Limit", default)]
    pub limit: Option<String>,
    /// 旧键 `Dashboard.Coin.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Coin.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Core` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardCore {
    /// 旧键 `Dashboard.Core.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Core.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Cube` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardCube {
    /// 旧键 `Dashboard.Cube.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Cube.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Gem` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardGem {
    /// 旧键 `Dashboard.Gem.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Gem.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.GuildCoin` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardGuildCoin {
    /// 旧键 `Dashboard.GuildCoin.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.GuildCoin.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Medal` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardMedal {
    /// 旧键 `Dashboard.Medal.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Medal.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Merit` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardMerit {
    /// 旧键 `Dashboard.Merit.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Merit.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Oil` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardOil {
    /// 旧键 `Dashboard.Oil.Limit`
    #[serde(rename = "Limit", default)]
    pub limit: Option<String>,
    /// 旧键 `Dashboard.Oil.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Oil.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.Pt` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardPt {
    /// 旧键 `Dashboard.Pt.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.Pt.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.PurpleCoin` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardPurpleCoin {
    /// 旧键 `Dashboard.PurpleCoin.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.PurpleCoin.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard.YellowCoin` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DashboardYellowCoin {
    /// 旧键 `Dashboard.YellowCoin.Value`
    #[serde(rename = "Value", default)]
    pub value: Option<String>,
    /// 旧键 `Dashboard.YellowCoin.Record`
    #[serde(rename = "Record", default)]
    pub record: Option<String>,
}

/// `Dashboard` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Dashboard {
    /// 旧键 `Dashboard.ActionPoint.*`
    #[serde(rename = "ActionPoint", default)]
    pub action_point: Option<DashboardActionPoint>,
    /// 旧键 `Dashboard.Coin.*`
    #[serde(rename = "Coin", default)]
    pub coin: Option<DashboardCoin>,
    /// 旧键 `Dashboard.Core.*`
    #[serde(rename = "Core", default)]
    pub core: Option<DashboardCore>,
    /// 旧键 `Dashboard.Cube.*`
    #[serde(rename = "Cube", default)]
    pub cube: Option<DashboardCube>,
    /// 旧键 `Dashboard.Gem.*`
    #[serde(rename = "Gem", default)]
    pub gem: Option<DashboardGem>,
    /// 旧键 `Dashboard.GuildCoin.*`
    #[serde(rename = "GuildCoin", default)]
    pub guild_coin: Option<DashboardGuildCoin>,
    /// 旧键 `Dashboard.Medal.*`
    #[serde(rename = "Medal", default)]
    pub medal: Option<DashboardMedal>,
    /// 旧键 `Dashboard.Merit.*`
    #[serde(rename = "Merit", default)]
    pub merit: Option<DashboardMerit>,
    /// 旧键 `Dashboard.Oil.*`
    #[serde(rename = "Oil", default)]
    pub oil: Option<DashboardOil>,
    /// 旧键 `Dashboard.Pt.*`
    #[serde(rename = "Pt", default)]
    pub pt: Option<DashboardPt>,
    /// 旧键 `Dashboard.PurpleCoin.*`
    #[serde(rename = "PurpleCoin", default)]
    pub purple_coin: Option<DashboardPurpleCoin>,
    /// 旧键 `Dashboard.YellowCoin.*`
    #[serde(rename = "YellowCoin", default)]
    pub yellow_coin: Option<DashboardYellowCoin>,
}

/// `Dorm.BuyFurniture` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DormBuyFurniture {
    /// 旧键 `Dorm.BuyFurniture.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Dorm.BuyFurniture.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `Dorm.BuyFurniture.BuyOption`
    #[serde(rename = "BuyOption", default)]
    pub buy_option: Option<String>,
}

/// `Dorm.Dorm` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DormDorm {
    /// 旧键 `Dorm.Dorm.Feed`
    #[serde(rename = "Feed", default)]
    pub feed: Option<bool>,
    /// 旧键 `Dorm.Dorm.BuyFood`
    #[serde(rename = "BuyFood", default)]
    pub buy_food: Option<bool>,
    /// 旧键 `Dorm.Dorm.Collect`
    #[serde(rename = "Collect", default)]
    pub collect: Option<bool>,
    /// 旧键 `Dorm.Dorm.FeedFilter`
    #[serde(rename = "FeedFilter", default)]
    pub feed_filter: Option<String>,
}

/// `Dorm.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DormScheduler {
    /// 旧键 `Dorm.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Dorm.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Dorm.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Dorm.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Dorm` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Dorm {
    /// 旧键 `Dorm.BuyFurniture.*`
    #[serde(rename = "BuyFurniture", default)]
    pub buy_furniture: Option<DormBuyFurniture>,
    /// 旧键 `Dorm.Dorm.*`
    #[serde(rename = "Dorm", default)]
    pub dorm: Option<DormDorm>,
    /// 旧键 `Dorm.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<DormScheduler>,
}

/// `EmulatorManager.EmulatorManager` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EmulatorManagerEmulatorManager {
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteSSHHost`
    #[serde(rename = "RemoteSSHHost", default)]
    pub remote_sshhost: Option<String>,
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteSSHPort`
    #[serde(rename = "RemoteSSHPort", default)]
    pub remote_sshport: Option<String>,
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteSSHUser`
    #[serde(rename = "RemoteSSHUser", default)]
    pub remote_sshuser: Option<String>,
    /// 旧键 `EmulatorManager.EmulatorManager.EnableRemoteSSH`
    #[serde(rename = "EnableRemoteSSH", default)]
    pub enable_remote_ssh: Option<bool>,
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteStopCommand`
    #[serde(rename = "RemoteStopCommand", default)]
    pub remote_stop_command: Option<String>,
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteSSHPublicKey`
    #[serde(rename = "RemoteSSHPublicKey", default)]
    pub remote_sshpublic_key: Option<String>,
    /// 旧键 `EmulatorManager.EmulatorManager.RemoteStartCommand`
    #[serde(rename = "RemoteStartCommand", default)]
    pub remote_start_command: Option<String>,
}

/// `EmulatorManager` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EmulatorManager {
    /// 旧键 `EmulatorManager.EmulatorManager.*`
    #[serde(rename = "EmulatorManager", default)]
    pub emulator_manager: Option<EmulatorManagerEmulatorManager>,
}

/// `Event.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCampaign {
    /// 旧键 `Event.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Event.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `Event.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Event.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Event.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Event.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Event.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Event.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Event.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventEmotion {
    /// 旧键 `Event.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Event.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Event.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Event.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Event.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Event.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Event.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Event.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Event.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Event.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Event.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Event.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Event.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventEnemyPriority {
    /// 旧键 `Event.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Event.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventFleet {
    /// 旧键 `Event.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Event.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Event.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Event.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Event.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Event.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Event.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Event.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Event.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Event.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Event.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventHpControl {
    /// 旧键 `Event.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Event.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Event.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Event.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Event.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Event.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Event.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Event.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Event.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventScheduler {
    /// 旧键 `Event.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Event.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Event.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Event.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Event.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventStopCondition {
    /// 旧键 `Event.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Event.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Event.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Event.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Event.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Event.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Event.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Event.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSubmarine {
    /// 旧键 `Event.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Event.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Event.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Event` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event {
    /// 旧键 `Event.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventCampaign>,
    /// 旧键 `Event.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventEmotion>,
    /// 旧键 `Event.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventEnemyPriority>,
    /// 旧键 `Event.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventFleet>,
    /// 旧键 `Event.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventHpControl>,
    /// 旧键 `Event.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventScheduler>,
    /// 旧键 `Event.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventStopCondition>,
    /// 旧键 `Event.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventSubmarine>,
}

/// `Event2.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2Campaign {
    /// 旧键 `Event2.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Event2.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `Event2.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Event2.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Event2.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Event2.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Event2.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Event2.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Event2.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2Emotion {
    /// 旧键 `Event2.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Event2.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Event2.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Event2.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Event2.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Event2.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Event2.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2EnemyPriority {
    /// 旧键 `Event2.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Event2.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2Fleet {
    /// 旧键 `Event2.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Event2.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Event2.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Event2.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Event2.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2HpControl {
    /// 旧键 `Event2.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Event2.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Event2.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Event2.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Event2.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Event2.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Event2.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Event2.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Event2.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2Scheduler {
    /// 旧键 `Event2.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Event2.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Event2.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Event2.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Event2.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2StopCondition {
    /// 旧键 `Event2.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Event2.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Event2.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Event2.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Event2.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Event2.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Event2.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Event2.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2Submarine {
    /// 旧键 `Event2.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event2.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Event2.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Event2.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Event2` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event2 {
    /// 旧键 `Event2.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<Event2Campaign>,
    /// 旧键 `Event2.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<Event2Emotion>,
    /// 旧键 `Event2.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<Event2EnemyPriority>,
    /// 旧键 `Event2.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<Event2Fleet>,
    /// 旧键 `Event2.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<Event2HpControl>,
    /// 旧键 `Event2.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<Event2Scheduler>,
    /// 旧键 `Event2.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<Event2StopCondition>,
    /// 旧键 `Event2.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<Event2Submarine>,
}

/// `Event3.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3Campaign {
    /// 旧键 `Event3.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Event3.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `Event3.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Event3.Campaign.AmbushEvade`
    #[serde(rename = "AmbushEvade", default)]
    pub ambush_evade: Option<bool>,
    /// 旧键 `Event3.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Event3.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Event3.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Event3.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Event3.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Event3.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3Emotion {
    /// 旧键 `Event3.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Event3.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Event3.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Event3.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Event3.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Event3.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Event3.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3EnemyPriority {
    /// 旧键 `Event3.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Event3.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3Fleet {
    /// 旧键 `Event3.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Event3.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Event3.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Event3.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Event3.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3HpControl {
    /// 旧键 `Event3.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Event3.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Event3.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Event3.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Event3.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Event3.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Event3.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Event3.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Event3.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3Scheduler {
    /// 旧键 `Event3.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Event3.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Event3.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Event3.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Event3.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3StopCondition {
    /// 旧键 `Event3.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Event3.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Event3.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Event3.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Event3.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Event3.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Event3.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Event3.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3Submarine {
    /// 旧键 `Event3.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Event3.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Event3.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Event3.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Event3` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event3 {
    /// 旧键 `Event3.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<Event3Campaign>,
    /// 旧键 `Event3.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<Event3Emotion>,
    /// 旧键 `Event3.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<Event3EnemyPriority>,
    /// 旧键 `Event3.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<Event3Fleet>,
    /// 旧键 `Event3.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<Event3HpControl>,
    /// 旧键 `Event3.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<Event3Scheduler>,
    /// 旧键 `Event3.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<Event3StopCondition>,
    /// 旧键 `Event3.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<Event3Submarine>,
}

/// `EventA.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventACampaign {
    /// 旧键 `EventA.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `EventA.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `EventA.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `EventA.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `EventA.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `EventA.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `EventA.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAEmotion {
    /// 旧键 `EventA.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `EventA.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `EventA.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `EventA.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `EventA.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `EventA.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `EventA.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAEnemyPriority {
    /// 旧键 `EventA.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `EventA.EventDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAEventDaily {
    /// 旧键 `EventA.EventDaily.LastStage`
    #[serde(rename = "LastStage", default)]
    pub last_stage: Option<String>,
    /// 旧键 `EventA.EventDaily.StageFilter`
    #[serde(rename = "StageFilter", default)]
    pub stage_filter: Option<String>,
}

/// `EventA.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAFleet {
    /// 旧键 `EventA.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `EventA.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `EventA.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `EventA.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `EventA.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAHpControl {
    /// 旧键 `EventA.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `EventA.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `EventA.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `EventA.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `EventA.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `EventA.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `EventA.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `EventA.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `EventA.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAScheduler {
    /// 旧键 `EventA.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventA.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventA.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventA.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventA.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventAStopCondition {
    /// 旧键 `EventA.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `EventA.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventA.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventASubmarine {
    /// 旧键 `EventA.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventA.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `EventA.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `EventA.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `EventA` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventA {
    /// 旧键 `EventA.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventACampaign>,
    /// 旧键 `EventA.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventAEmotion>,
    /// 旧键 `EventA.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventAEnemyPriority>,
    /// 旧键 `EventA.EventDaily.*`
    #[serde(rename = "EventDaily", default)]
    pub event_daily: Option<EventAEventDaily>,
    /// 旧键 `EventA.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventAFleet>,
    /// 旧键 `EventA.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventAHpControl>,
    /// 旧键 `EventA.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventAScheduler>,
    /// 旧键 `EventA.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventAStopCondition>,
    /// 旧键 `EventA.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventASubmarine>,
}

/// `EventB.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBCampaign {
    /// 旧键 `EventB.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `EventB.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `EventB.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `EventB.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `EventB.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `EventB.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `EventB.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBEmotion {
    /// 旧键 `EventB.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `EventB.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `EventB.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `EventB.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `EventB.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `EventB.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `EventB.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBEnemyPriority {
    /// 旧键 `EventB.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `EventB.EventDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBEventDaily {
    /// 旧键 `EventB.EventDaily.LastStage`
    #[serde(rename = "LastStage", default)]
    pub last_stage: Option<String>,
    /// 旧键 `EventB.EventDaily.StageFilter`
    #[serde(rename = "StageFilter", default)]
    pub stage_filter: Option<String>,
}

/// `EventB.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBFleet {
    /// 旧键 `EventB.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `EventB.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `EventB.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `EventB.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `EventB.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBHpControl {
    /// 旧键 `EventB.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `EventB.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `EventB.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `EventB.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `EventB.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `EventB.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `EventB.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `EventB.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `EventB.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBScheduler {
    /// 旧键 `EventB.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventB.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventB.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventB.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventB.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBStopCondition {
    /// 旧键 `EventB.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `EventB.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventB.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventBSubmarine {
    /// 旧键 `EventB.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventB.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `EventB.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `EventB.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `EventB` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventB {
    /// 旧键 `EventB.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventBCampaign>,
    /// 旧键 `EventB.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventBEmotion>,
    /// 旧键 `EventB.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventBEnemyPriority>,
    /// 旧键 `EventB.EventDaily.*`
    #[serde(rename = "EventDaily", default)]
    pub event_daily: Option<EventBEventDaily>,
    /// 旧键 `EventB.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventBFleet>,
    /// 旧键 `EventB.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventBHpControl>,
    /// 旧键 `EventB.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventBScheduler>,
    /// 旧键 `EventB.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventBStopCondition>,
    /// 旧键 `EventB.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventBSubmarine>,
}

/// `EventC.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCCampaign {
    /// 旧键 `EventC.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `EventC.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `EventC.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `EventC.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `EventC.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `EventC.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `EventC.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCEmotion {
    /// 旧键 `EventC.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `EventC.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `EventC.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `EventC.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `EventC.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `EventC.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `EventC.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCEnemyPriority {
    /// 旧键 `EventC.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `EventC.EventDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCEventDaily {
    /// 旧键 `EventC.EventDaily.LastStage`
    #[serde(rename = "LastStage", default)]
    pub last_stage: Option<String>,
    /// 旧键 `EventC.EventDaily.StageFilter`
    #[serde(rename = "StageFilter", default)]
    pub stage_filter: Option<String>,
}

/// `EventC.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCFleet {
    /// 旧键 `EventC.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `EventC.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `EventC.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `EventC.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `EventC.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCHpControl {
    /// 旧键 `EventC.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `EventC.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `EventC.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `EventC.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `EventC.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `EventC.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `EventC.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `EventC.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `EventC.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCScheduler {
    /// 旧键 `EventC.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventC.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventC.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventC.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventC.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCStopCondition {
    /// 旧键 `EventC.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `EventC.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventC.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventCSubmarine {
    /// 旧键 `EventC.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventC.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `EventC.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `EventC.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `EventC` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventC {
    /// 旧键 `EventC.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventCCampaign>,
    /// 旧键 `EventC.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventCEmotion>,
    /// 旧键 `EventC.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventCEnemyPriority>,
    /// 旧键 `EventC.EventDaily.*`
    #[serde(rename = "EventDaily", default)]
    pub event_daily: Option<EventCEventDaily>,
    /// 旧键 `EventC.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventCFleet>,
    /// 旧键 `EventC.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventCHpControl>,
    /// 旧键 `EventC.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventCScheduler>,
    /// 旧键 `EventC.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventCStopCondition>,
    /// 旧键 `EventC.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventCSubmarine>,
}

/// `EventD.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDCampaign {
    /// 旧键 `EventD.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `EventD.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `EventD.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `EventD.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `EventD.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `EventD.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `EventD.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDEmotion {
    /// 旧键 `EventD.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `EventD.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `EventD.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `EventD.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `EventD.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `EventD.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `EventD.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDEnemyPriority {
    /// 旧键 `EventD.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `EventD.EventDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDEventDaily {
    /// 旧键 `EventD.EventDaily.LastStage`
    #[serde(rename = "LastStage", default)]
    pub last_stage: Option<String>,
    /// 旧键 `EventD.EventDaily.StageFilter`
    #[serde(rename = "StageFilter", default)]
    pub stage_filter: Option<String>,
}

/// `EventD.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDFleet {
    /// 旧键 `EventD.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `EventD.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `EventD.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `EventD.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `EventD.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDHpControl {
    /// 旧键 `EventD.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `EventD.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `EventD.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `EventD.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `EventD.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `EventD.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `EventD.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `EventD.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `EventD.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDScheduler {
    /// 旧键 `EventD.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventD.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventD.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventD.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventD.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDStopCondition {
    /// 旧键 `EventD.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `EventD.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventD.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventDSubmarine {
    /// 旧键 `EventD.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventD.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `EventD.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `EventD.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `EventD` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventD {
    /// 旧键 `EventD.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventDCampaign>,
    /// 旧键 `EventD.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventDEmotion>,
    /// 旧键 `EventD.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventDEnemyPriority>,
    /// 旧键 `EventD.EventDaily.*`
    #[serde(rename = "EventDaily", default)]
    pub event_daily: Option<EventDEventDaily>,
    /// 旧键 `EventD.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventDFleet>,
    /// 旧键 `EventD.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventDHpControl>,
    /// 旧键 `EventD.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventDScheduler>,
    /// 旧键 `EventD.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventDStopCondition>,
    /// 旧键 `EventD.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventDSubmarine>,
}

/// `EventGeneral.EventGeneral` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventGeneralEventGeneral {
    /// 旧键 `EventGeneral.EventGeneral.PtLimit`
    #[serde(rename = "PtLimit", default)]
    pub pt_limit: Option<String>,
    /// 旧键 `EventGeneral.EventGeneral.TimeLimit`
    #[serde(rename = "TimeLimit", default)]
    pub time_limit: Option<String>,
}

/// `EventGeneral.TaskBalancer` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventGeneralTaskBalancer {
    /// 旧键 `EventGeneral.TaskBalancer.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventGeneral.TaskBalancer.TaskCall`
    #[serde(rename = "TaskCall", default)]
    pub task_call: Option<String>,
    /// 旧键 `EventGeneral.TaskBalancer.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventGeneral` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventGeneral {
    /// 旧键 `EventGeneral.EventGeneral.*`
    #[serde(rename = "EventGeneral", default)]
    pub event_general: Option<EventGeneralEventGeneral>,
    /// 旧键 `EventGeneral.TaskBalancer.*`
    #[serde(rename = "TaskBalancer", default)]
    pub task_balancer: Option<EventGeneralTaskBalancer>,
}

/// `EventShop.EventShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventShopEventShop {
    /// 旧键 `EventShop.EventShop.BuyURShip`
    #[serde(rename = "BuyURShip", default)]
    pub buy_urship: Option<String>,
    /// 旧键 `EventShop.EventShop.CustomFilter`
    #[serde(rename = "CustomFilter", default)]
    pub custom_filter: Option<String>,
    /// 旧键 `EventShop.EventShop.PresetFilter`
    #[serde(rename = "PresetFilter", default)]
    pub preset_filter: Option<String>,
    /// 旧键 `EventShop.EventShop.UnlockSSRShip`
    #[serde(rename = "UnlockSSRShip", default)]
    pub unlock_ssrship: Option<bool>,
}

/// `EventShop.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventShopScheduler {
    /// 旧键 `EventShop.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventShop.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventShop.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventShop.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventShop` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventShop {
    /// 旧键 `EventShop.EventShop.*`
    #[serde(rename = "EventShop", default)]
    pub event_shop: Option<EventShopEventShop>,
    /// 旧键 `EventShop.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventShopScheduler>,
}

/// `EventSp.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpCampaign {
    /// 旧键 `EventSp.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `EventSp.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `EventSp.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `EventSp.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `EventSp.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `EventSp.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `EventSp.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpEmotion {
    /// 旧键 `EventSp.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `EventSp.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `EventSp.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `EventSp.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `EventSp.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `EventSp.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `EventSp.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpEnemyPriority {
    /// 旧键 `EventSp.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `EventSp.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpFleet {
    /// 旧键 `EventSp.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `EventSp.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `EventSp.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `EventSp.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `EventSp.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpHpControl {
    /// 旧键 `EventSp.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `EventSp.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `EventSp.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `EventSp.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `EventSp.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `EventSp.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `EventSp.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `EventSp.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `EventSp.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpScheduler {
    /// 旧键 `EventSp.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `EventSp.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `EventSp.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `EventSp.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `EventSp.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpStopCondition {
    /// 旧键 `EventSp.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `EventSp.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `EventSp.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSpSubmarine {
    /// 旧键 `EventSp.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `EventSp.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `EventSp.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `EventSp` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct EventSp {
    /// 旧键 `EventSp.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<EventSpCampaign>,
    /// 旧键 `EventSp.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<EventSpEmotion>,
    /// 旧键 `EventSp.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<EventSpEnemyPriority>,
    /// 旧键 `EventSp.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<EventSpFleet>,
    /// 旧键 `EventSp.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<EventSpHpControl>,
    /// 旧键 `EventSp.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<EventSpScheduler>,
    /// 旧键 `EventSp.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<EventSpStopCondition>,
    /// 旧键 `EventSp.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<EventSpSubmarine>,
}

/// `Exercise.Exercise` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ExerciseExercise {
    /// 旧键 `Exercise.Exercise.OpponentTrial`
    #[serde(rename = "OpponentTrial", default)]
    pub opponent_trial: Option<String>,
    /// 旧键 `Exercise.Exercise.ExerciseStrategy`
    #[serde(rename = "ExerciseStrategy", default)]
    pub exercise_strategy: Option<String>,
    /// 旧键 `Exercise.Exercise.LowHpConfirmWait`
    #[serde(rename = "LowHpConfirmWait", default)]
    pub low_hp_confirm_wait: Option<String>,
    /// 旧键 `Exercise.Exercise.OpponentChooseMode`
    #[serde(rename = "OpponentChooseMode", default)]
    pub opponent_choose_mode: Option<String>,
    /// 旧键 `Exercise.Exercise.OpponentRefreshValue`
    #[serde(rename = "OpponentRefreshValue", default)]
    pub opponent_refresh_value: Option<String>,
    /// 旧键 `Exercise.Exercise.OpponentRefreshRecord`
    #[serde(rename = "OpponentRefreshRecord", default)]
    pub opponent_refresh_record: Option<String>,
    /// 旧键 `Exercise.Exercise.DelayUntilHoursBeforeNextUpdate`
    #[serde(rename = "DelayUntilHoursBeforeNextUpdate", default)]
    pub delay_until_hours_before_next_update: Option<String>,
    /// 旧键 `Exercise.Exercise.LowHpThreshold`
    #[serde(rename = "LowHpThreshold", default)]
    pub low_hp_threshold: Option<String>,
}

/// `Exercise.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ExerciseScheduler {
    /// 旧键 `Exercise.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Exercise.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Exercise.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Exercise.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Exercise` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Exercise {
    /// 旧键 `Exercise.Exercise.*`
    #[serde(rename = "Exercise", default)]
    pub exercise: Option<ExerciseExercise>,
    /// 旧键 `Exercise.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ExerciseScheduler>,
}

/// `Freebies.BattlePass` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FreebiesBattlePass {
    /// 旧键 `Freebies.BattlePass.Collect`
    #[serde(rename = "Collect", default)]
    pub collect: Option<bool>,
}

/// `Freebies.DataKey` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FreebiesDataKey {
    /// 旧键 `Freebies.DataKey.Collect`
    #[serde(rename = "Collect", default)]
    pub collect: Option<bool>,
    /// 旧键 `Freebies.DataKey.ForceCollect`
    #[serde(rename = "ForceCollect", default)]
    pub force_collect: Option<bool>,
}

/// `Freebies.Mail` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FreebiesMail {
    /// 旧键 `Freebies.Mail.ClaimMerit`
    #[serde(rename = "ClaimMerit", default)]
    pub claim_merit: Option<bool>,
    /// 旧键 `Freebies.Mail.DeleteCollected`
    #[serde(rename = "DeleteCollected", default)]
    pub delete_collected: Option<bool>,
    /// 旧键 `Freebies.Mail.ClaimMaintenance`
    #[serde(rename = "ClaimMaintenance", default)]
    pub claim_maintenance: Option<bool>,
    /// 旧键 `Freebies.Mail.ClaimTradeLicense`
    #[serde(rename = "ClaimTradeLicense", default)]
    pub claim_trade_license: Option<bool>,
}

/// `Freebies.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FreebiesScheduler {
    /// 旧键 `Freebies.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Freebies.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Freebies.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Freebies.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Freebies.SupplyPack` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FreebiesSupplyPack {
    /// 旧键 `Freebies.SupplyPack.Collect`
    #[serde(rename = "Collect", default)]
    pub collect: Option<bool>,
    /// 旧键 `Freebies.SupplyPack.DayOfWeek`
    #[serde(rename = "DayOfWeek", default)]
    pub day_of_week: Option<String>,
}

/// `Freebies` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Freebies {
    /// 旧键 `Freebies.BattlePass.*`
    #[serde(rename = "BattlePass", default)]
    pub battle_pass: Option<FreebiesBattlePass>,
    /// 旧键 `Freebies.DataKey.*`
    #[serde(rename = "DataKey", default)]
    pub data_key: Option<FreebiesDataKey>,
    /// 旧键 `Freebies.Mail.*`
    #[serde(rename = "Mail", default)]
    pub mail: Option<FreebiesMail>,
    /// 旧键 `Freebies.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<FreebiesScheduler>,
    /// 旧键 `Freebies.SupplyPack.*`
    #[serde(rename = "SupplyPack", default)]
    pub supply_pack: Option<FreebiesSupplyPack>,
}

/// `Gacha.Gacha` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GachaGacha {
    /// 旧键 `Gacha.Gacha.Pool`
    #[serde(rename = "Pool", default)]
    pub pool: Option<String>,
    /// 旧键 `Gacha.Gacha.Amount`
    #[serde(rename = "Amount", default)]
    pub amount: Option<String>,
    /// 旧键 `Gacha.Gacha.UseDrill`
    #[serde(rename = "UseDrill", default)]
    pub use_drill: Option<bool>,
    /// 旧键 `Gacha.Gacha.UseTicket`
    #[serde(rename = "UseTicket", default)]
    pub use_ticket: Option<bool>,
}

/// `Gacha.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GachaScheduler {
    /// 旧键 `Gacha.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Gacha.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Gacha.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Gacha.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Gacha` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Gacha {
    /// 旧键 `Gacha.Gacha.*`
    #[serde(rename = "Gacha", default)]
    pub gacha: Option<GachaGacha>,
    /// 旧键 `Gacha.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<GachaScheduler>,
}

/// `GameManager.GameManager` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GameManagerGameManager {
    /// 旧键 `GameManager.GameManager.AutoRestart`
    #[serde(rename = "AutoRestart", default)]
    pub auto_restart: Option<bool>,
}

/// `GameManager` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GameManager {
    /// 旧键 `GameManager.GameManager.*`
    #[serde(rename = "GameManager", default)]
    pub game_manager: Option<GameManagerGameManager>,
}

/// `GemsFarming.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingCampaign {
    /// 旧键 `GemsFarming.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `GemsFarming.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `GemsFarming.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `GemsFarming.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `GemsFarming.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `GemsFarming.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingEmotion {
    /// 旧键 `GemsFarming.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `GemsFarming.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `GemsFarming.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `GemsFarming.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
}

/// `GemsFarming.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingFleet {
    /// 旧键 `GemsFarming.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `GemsFarming.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `GemsFarming.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `GemsFarming.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `GemsFarming.GemsFarming` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingGemsFarming {
    /// 旧键 `GemsFarming.GemsFarming.CommonCV`
    #[serde(rename = "CommonCV", default)]
    pub common_cv: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.CommonDD`
    #[serde(rename = "CommonDD", default)]
    pub common_dd: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.EquipmentCode`
    #[serde(rename = "EquipmentCode", default)]
    pub equipment_code: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.ChangeFlagship`
    #[serde(rename = "ChangeFlagship", default)]
    pub change_flagship: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.ChangeVanguard`
    #[serde(rename = "ChangeVanguard", default)]
    pub change_vanguard: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.CommonCVFilter`
    #[serde(rename = "CommonCVFilter", default)]
    pub common_cvfilter: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.CommonDDFilter`
    #[serde(rename = "CommonDDFilter", default)]
    pub common_ddfilter: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.CommissionLimit`
    #[serde(rename = "CommissionLimit", default)]
    pub commission_limit: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.UseEmotionFirst`
    #[serde(rename = "UseEmotionFirst", default)]
    pub use_emotion_first: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.VanguardLevelMax`
    #[serde(rename = "VanguardLevelMax", default)]
    pub vanguard_level_max: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.VanguardLevelMin`
    #[serde(rename = "VanguardLevelMin", default)]
    pub vanguard_level_min: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.IgnoreEmotionWarning`
    #[serde(rename = "IgnoreEmotionWarning", default)]
    pub ignore_emotion_warning: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.AllowLowVanguardLevel`
    #[serde(rename = "AllowLowVanguardLevel", default)]
    pub allow_low_vanguard_level: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.DelayTaskIFNoFlagship`
    #[serde(rename = "DelayTaskIFNoFlagship", default)]
    pub delay_task_ifno_flagship: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.AllowHighFlagshipLevel`
    #[serde(rename = "AllowHighFlagshipLevel", default)]
    pub allow_high_flagship_level: Option<bool>,
    /// 旧键 `GemsFarming.GemsFarming.HighValueCommissionReserve`
    #[serde(rename = "HighValueCommissionReserve", default)]
    pub high_value_commission_reserve: Option<String>,
    /// 旧键 `GemsFarming.GemsFarming.HighValueCommissionFilterCount`
    #[serde(rename = "HighValueCommissionFilterCount", default)]
    pub high_value_commission_filter_count: Option<String>,
}

/// `GemsFarming.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingScheduler {
    /// 旧键 `GemsFarming.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `GemsFarming.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `GemsFarming.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `GemsFarming.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `GemsFarming.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingStopCondition {
    /// 旧键 `GemsFarming.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `GemsFarming.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `GemsFarming.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarmingSubmarine {
    /// 旧键 `GemsFarming.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `GemsFarming.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `GemsFarming` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GemsFarming {
    /// 旧键 `GemsFarming.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<GemsFarmingCampaign>,
    /// 旧键 `GemsFarming.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<GemsFarmingEmotion>,
    /// 旧键 `GemsFarming.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<GemsFarmingFleet>,
    /// 旧键 `GemsFarming.GemsFarming.*`
    #[serde(rename = "GemsFarming", default)]
    pub gems_farming: Option<GemsFarmingGemsFarming>,
    /// 旧键 `GemsFarming.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<GemsFarmingScheduler>,
    /// 旧键 `GemsFarming.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<GemsFarmingStopCondition>,
    /// 旧键 `GemsFarming.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<GemsFarmingSubmarine>,
}

/// `General.Enhance` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralEnhance {
    /// 旧键 `General.Enhance.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
    /// 旧键 `General.Enhance.ShipToEnhance`
    #[serde(rename = "ShipToEnhance", default)]
    pub ship_to_enhance: Option<String>,
    /// 旧键 `General.Enhance.CheckPerCategory`
    #[serde(rename = "CheckPerCategory", default)]
    pub check_per_category: Option<String>,
}

/// `General.Log` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralLog {
    /// 旧键 `General.Log.ZipMethod`
    #[serde(rename = "ZipMethod", default)]
    pub zip_method: Option<String>,
    /// 旧键 `General.Log.LogKeepCount`
    #[serde(rename = "LogKeepCount", default)]
    pub log_keep_count: Option<String>,
    /// 旧键 `General.Log.LogBackUpMethod`
    #[serde(rename = "LogBackUpMethod", default)]
    pub log_back_up_method: Option<String>,
}

/// `General.OldRetire` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralOldRetire {
    /// 旧键 `General.OldRetire.N`
    #[serde(rename = "N", default)]
    pub n: Option<bool>,
    /// 旧键 `General.OldRetire.R`
    #[serde(rename = "R", default)]
    pub r: Option<bool>,
    /// 旧键 `General.OldRetire.SR`
    #[serde(rename = "SR", default)]
    pub sr: Option<bool>,
    /// 旧键 `General.OldRetire.SSR`
    #[serde(rename = "SSR", default)]
    pub ssr: Option<bool>,
    /// 旧键 `General.OldRetire.RetireAmount`
    #[serde(rename = "RetireAmount", default)]
    pub retire_amount: Option<String>,
}

/// `General.OneClickRetire` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralOneClickRetire {
    /// 旧键 `General.OneClickRetire.KeepLimitBreak`
    #[serde(rename = "KeepLimitBreak", default)]
    pub keep_limit_break: Option<String>,
}

/// `General.PublicEmotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralPublicEmotion {
    /// 旧键 `General.PublicEmotion.Tasks`
    #[serde(rename = "Tasks", default)]
    pub tasks: Option<String>,
    /// 旧键 `General.PublicEmotion.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `General.PublicEmotion.FleetOath`
    #[serde(rename = "FleetOath", default)]
    pub fleet_oath: Option<bool>,
    /// 旧键 `General.PublicEmotion.FleetOnsen`
    #[serde(rename = "FleetOnsen", default)]
    pub fleet_onsen: Option<bool>,
    /// 旧键 `General.PublicEmotion.FleetValue`
    #[serde(rename = "FleetValue", default)]
    pub fleet_value: Option<String>,
    /// 旧键 `General.PublicEmotion.FleetRecord`
    #[serde(rename = "FleetRecord", default)]
    pub fleet_record: Option<String>,
    /// 旧键 `General.PublicEmotion.FleetControl`
    #[serde(rename = "FleetControl", default)]
    pub fleet_control: Option<String>,
    /// 旧键 `General.PublicEmotion.FleetRecover`
    #[serde(rename = "FleetRecover", default)]
    pub fleet_recover: Option<String>,
}

/// `General.Retirement` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GeneralRetirement {
    /// 旧键 `General.Retirement.RetireMode`
    #[serde(rename = "RetireMode", default)]
    pub retire_mode: Option<String>,
}

/// `General` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct General {
    /// 旧键 `General.Enhance.*`
    #[serde(rename = "Enhance", default)]
    pub enhance: Option<GeneralEnhance>,
    /// 旧键 `General.Log.*`
    #[serde(rename = "Log", default)]
    pub log: Option<GeneralLog>,
    /// 旧键 `General.OldRetire.*`
    #[serde(rename = "OldRetire", default)]
    pub old_retire: Option<GeneralOldRetire>,
    /// 旧键 `General.OneClickRetire.*`
    #[serde(rename = "OneClickRetire", default)]
    pub one_click_retire: Option<GeneralOneClickRetire>,
    /// 旧键 `General.PublicEmotion.*`
    #[serde(rename = "PublicEmotion", default)]
    pub public_emotion: Option<GeneralPublicEmotion>,
    /// 旧键 `General.Retirement.*`
    #[serde(rename = "Retirement", default)]
    pub retirement: Option<GeneralRetirement>,
}

/// `Guild.GuildLogistics` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GuildGuildLogistics {
    /// 旧键 `Guild.GuildLogistics.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Guild.GuildLogistics.ExchangeFilter`
    #[serde(rename = "ExchangeFilter", default)]
    pub exchange_filter: Option<String>,
    /// 旧键 `Guild.GuildLogistics.SelectNewMission`
    #[serde(rename = "SelectNewMission", default)]
    pub select_new_mission: Option<bool>,
}

/// `Guild.GuildOperation` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GuildGuildOperation {
    /// 旧键 `Guild.GuildOperation.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Guild.GuildOperation.AttackBoss`
    #[serde(rename = "AttackBoss", default)]
    pub attack_boss: Option<bool>,
    /// 旧键 `Guild.GuildOperation.BossFleetRecommend`
    #[serde(rename = "BossFleetRecommend", default)]
    pub boss_fleet_recommend: Option<bool>,
    /// 旧键 `Guild.GuildOperation.SelectNewOperation`
    #[serde(rename = "SelectNewOperation", default)]
    pub select_new_operation: Option<bool>,
    /// 旧键 `Guild.GuildOperation.NewOperationMaxDate`
    #[serde(rename = "NewOperationMaxDate", default)]
    pub new_operation_max_date: Option<String>,
    /// 旧键 `Guild.GuildOperation.JoinThreshold`
    #[serde(rename = "JoinThreshold", default)]
    pub join_threshold: Option<String>,
}

/// `Guild.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct GuildScheduler {
    /// 旧键 `Guild.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Guild.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Guild.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Guild.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Guild` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Guild {
    /// 旧键 `Guild.GuildLogistics.*`
    #[serde(rename = "GuildLogistics", default)]
    pub guild_logistics: Option<GuildGuildLogistics>,
    /// 旧键 `Guild.GuildOperation.*`
    #[serde(rename = "GuildOperation", default)]
    pub guild_operation: Option<GuildGuildOperation>,
    /// 旧键 `Guild.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<GuildScheduler>,
}

/// `Hard.Hard` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HardHard {
    /// 旧键 `Hard.Hard.HardFleet`
    #[serde(rename = "HardFleet", default)]
    pub hard_fleet: Option<String>,
    /// 旧键 `Hard.Hard.HardStage`
    #[serde(rename = "HardStage", default)]
    pub hard_stage: Option<String>,
}

/// `Hard.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HardScheduler {
    /// 旧键 `Hard.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Hard.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Hard.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Hard.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Hard` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Hard {
    /// 旧键 `Hard.Hard.*`
    #[serde(rename = "Hard", default)]
    pub hard: Option<HardHard>,
    /// 旧键 `Hard.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<HardScheduler>,
}

/// `Hospital.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HospitalEmotion {
    /// 旧键 `Hospital.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Hospital.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Hospital.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
}

/// `Hospital.Hospital` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HospitalHospital {
    /// 旧键 `Hospital.Hospital.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Hospital.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HospitalScheduler {
    /// 旧键 `Hospital.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Hospital.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Hospital.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Hospital.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Hospital.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct HospitalStopCondition {
    /// 旧键 `Hospital.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Hospital.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `Hospital` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Hospital {
    /// 旧键 `Hospital.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<HospitalEmotion>,
    /// 旧键 `Hospital.Hospital.*`
    #[serde(rename = "Hospital", default)]
    pub hospital: Option<HospitalHospital>,
    /// 旧键 `Hospital.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<HospitalScheduler>,
    /// 旧键 `Hospital.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<HospitalStopCondition>,
}

/// `IslandAirDrop.IslandAirDrop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandAirDropIslandAirDrop {
    /// 旧键 `IslandAirDrop.IslandAirDrop.LastSteal`
    #[serde(rename = "LastSteal", default)]
    pub last_steal: Option<String>,
    /// 旧键 `IslandAirDrop.IslandAirDrop.VisitOtherIsland`
    #[serde(rename = "VisitOtherIsland", default)]
    pub visit_other_island: Option<bool>,
}

/// `IslandAirDrop.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandAirDropScheduler {
    /// 旧键 `IslandAirDrop.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandAirDrop.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandAirDrop.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandAirDrop.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandAirDrop` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandAirDrop {
    /// 旧键 `IslandAirDrop.IslandAirDrop.*`
    #[serde(rename = "IslandAirDrop", default)]
    pub island_air_drop: Option<IslandAirDropIslandAirDrop>,
    /// 旧键 `IslandAirDrop.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandAirDropScheduler>,
}

/// `IslandBusiness.IslandBusiness` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusiness {
    /// 旧键 `IslandBusiness.IslandBusiness.BatchEnabled`
    #[serde(rename = "BatchEnabled", default)]
    pub batch_enabled: Option<bool>,
    /// 旧键 `IslandBusiness.IslandBusiness.SeasonalReplaceEnabled`
    #[serde(rename = "SeasonalReplaceEnabled", default)]
    pub seasonal_replace_enabled: Option<bool>,
    /// 旧键 `IslandBusiness.IslandBusiness.SeasonalThreshold`
    #[serde(rename = "SeasonalThreshold", default)]
    pub seasonal_threshold: Option<String>,
}

/// `IslandBusiness.IslandBusinessShop1` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusinessShop1 {
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Char1`
    #[serde(rename = "Char1", default)]
    pub char1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Char2`
    #[serde(rename = "Char2", default)]
    pub char2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Product1`
    #[serde(rename = "Product1", default)]
    pub product1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Product2`
    #[serde(rename = "Product2", default)]
    pub product2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Product3`
    #[serde(rename = "Product3", default)]
    pub product3: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Product4`
    #[serde(rename = "Product4", default)]
    pub product4: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.Product5`
    #[serde(rename = "Product5", default)]
    pub product5: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.SeasonalFallback`
    #[serde(rename = "SeasonalFallback", default)]
    pub seasonal_fallback: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.BoostReplaceFilter`
    #[serde(rename = "BoostReplaceFilter", default)]
    pub boost_replace_filter: Option<String>,
}

/// `IslandBusiness.IslandBusinessShop2` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusinessShop2 {
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Char1`
    #[serde(rename = "Char1", default)]
    pub char1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Char2`
    #[serde(rename = "Char2", default)]
    pub char2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Product1`
    #[serde(rename = "Product1", default)]
    pub product1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Product2`
    #[serde(rename = "Product2", default)]
    pub product2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Product3`
    #[serde(rename = "Product3", default)]
    pub product3: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Product4`
    #[serde(rename = "Product4", default)]
    pub product4: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.Product5`
    #[serde(rename = "Product5", default)]
    pub product5: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.SeasonalFallback`
    #[serde(rename = "SeasonalFallback", default)]
    pub seasonal_fallback: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.BoostReplaceFilter`
    #[serde(rename = "BoostReplaceFilter", default)]
    pub boost_replace_filter: Option<String>,
}

/// `IslandBusiness.IslandBusinessShop3` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusinessShop3 {
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Char1`
    #[serde(rename = "Char1", default)]
    pub char1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Char2`
    #[serde(rename = "Char2", default)]
    pub char2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Product1`
    #[serde(rename = "Product1", default)]
    pub product1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Product2`
    #[serde(rename = "Product2", default)]
    pub product2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Product3`
    #[serde(rename = "Product3", default)]
    pub product3: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Product4`
    #[serde(rename = "Product4", default)]
    pub product4: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.Product5`
    #[serde(rename = "Product5", default)]
    pub product5: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.BoostReplaceFilter`
    #[serde(rename = "BoostReplaceFilter", default)]
    pub boost_replace_filter: Option<String>,
}

/// `IslandBusiness.IslandBusinessShop4` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusinessShop4 {
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Char1`
    #[serde(rename = "Char1", default)]
    pub char1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Char2`
    #[serde(rename = "Char2", default)]
    pub char2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Product1`
    #[serde(rename = "Product1", default)]
    pub product1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Product2`
    #[serde(rename = "Product2", default)]
    pub product2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Product3`
    #[serde(rename = "Product3", default)]
    pub product3: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Product4`
    #[serde(rename = "Product4", default)]
    pub product4: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.Product5`
    #[serde(rename = "Product5", default)]
    pub product5: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.BoostReplaceFilter`
    #[serde(rename = "BoostReplaceFilter", default)]
    pub boost_replace_filter: Option<String>,
}

/// `IslandBusiness.IslandBusinessShop5` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessIslandBusinessShop5 {
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Char1`
    #[serde(rename = "Char1", default)]
    pub char1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Char2`
    #[serde(rename = "Char2", default)]
    pub char2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Product1`
    #[serde(rename = "Product1", default)]
    pub product1: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Product2`
    #[serde(rename = "Product2", default)]
    pub product2: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Product3`
    #[serde(rename = "Product3", default)]
    pub product3: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Product4`
    #[serde(rename = "Product4", default)]
    pub product4: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.Product5`
    #[serde(rename = "Product5", default)]
    pub product5: Option<String>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.BoostReplaceFilter`
    #[serde(rename = "BoostReplaceFilter", default)]
    pub boost_replace_filter: Option<String>,
}

/// `IslandBusiness.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusinessScheduler {
    /// 旧键 `IslandBusiness.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandBusiness.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandBusiness.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandBusiness.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandBusiness` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandBusiness {
    /// 旧键 `IslandBusiness.IslandBusiness.*`
    #[serde(rename = "IslandBusiness", default)]
    pub island_business: Option<IslandBusinessIslandBusiness>,
    /// 旧键 `IslandBusiness.IslandBusinessShop1.*`
    #[serde(rename = "IslandBusinessShop1", default)]
    pub island_business_shop1: Option<IslandBusinessIslandBusinessShop1>,
    /// 旧键 `IslandBusiness.IslandBusinessShop2.*`
    #[serde(rename = "IslandBusinessShop2", default)]
    pub island_business_shop2: Option<IslandBusinessIslandBusinessShop2>,
    /// 旧键 `IslandBusiness.IslandBusinessShop3.*`
    #[serde(rename = "IslandBusinessShop3", default)]
    pub island_business_shop3: Option<IslandBusinessIslandBusinessShop3>,
    /// 旧键 `IslandBusiness.IslandBusinessShop4.*`
    #[serde(rename = "IslandBusinessShop4", default)]
    pub island_business_shop4: Option<IslandBusinessIslandBusinessShop4>,
    /// 旧键 `IslandBusiness.IslandBusinessShop5.*`
    #[serde(rename = "IslandBusinessShop5", default)]
    pub island_business_shop5: Option<IslandBusinessIslandBusinessShop5>,
    /// 旧键 `IslandBusiness.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandBusinessScheduler>,
}

/// `IslandCargoPreparation.IslandCargoPreparation` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandCargoPreparationIslandCargoPreparation {
    /// 旧键 `IslandCargoPreparation.IslandCargoPreparation.Blacklist`
    #[serde(rename = "Blacklist", default)]
    pub blacklist: Option<String>,
}

/// `IslandCargoPreparation.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandCargoPreparationScheduler {
    /// 旧键 `IslandCargoPreparation.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandCargoPreparation.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandCargoPreparation.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandCargoPreparation.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandCargoPreparation` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandCargoPreparation {
    /// 旧键 `IslandCargoPreparation.IslandCargoPreparation.*`
    #[serde(rename = "IslandCargoPreparation", default)]
    pub island_cargo_preparation: Option<IslandCargoPreparationIslandCargoPreparation>,
    /// 旧键 `IslandCargoPreparation.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandCargoPreparationScheduler>,
}

/// `IslandDailyGather.IslandDailyGather` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyGatherIslandDailyGather {
    /// 旧键 `IslandDailyGather.IslandDailyGather.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
}

/// `IslandDailyGather.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyGatherScheduler {
    /// 旧键 `IslandDailyGather.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandDailyGather.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandDailyGather.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandDailyGather.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandDailyGather` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyGather {
    /// 旧键 `IslandDailyGather.IslandDailyGather.*`
    #[serde(rename = "IslandDailyGather", default)]
    pub island_daily_gather: Option<IslandDailyGatherIslandDailyGather>,
    /// 旧键 `IslandDailyGather.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandDailyGatherScheduler>,
}

/// `IslandDailyInteract.IslandDailyInteract` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyInteractIslandDailyInteract {
    /// 旧键 `IslandDailyInteract.IslandDailyInteract.WeeklyPhoto`
    #[serde(rename = "WeeklyPhoto", default)]
    pub weekly_photo: Option<bool>,
}

/// `IslandDailyInteract.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyInteractScheduler {
    /// 旧键 `IslandDailyInteract.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandDailyInteract.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandDailyInteract.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandDailyInteract.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandDailyInteract` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyInteract {
    /// 旧键 `IslandDailyInteract.IslandDailyInteract.*`
    #[serde(rename = "IslandDailyInteract", default)]
    pub island_daily_interact: Option<IslandDailyInteractIslandDailyInteract>,
    /// 旧键 `IslandDailyInteract.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandDailyInteractScheduler>,
}

/// `IslandDailyOrder.IslandDailyOrder` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyOrderIslandDailyOrder {
    /// 旧键 `IslandDailyOrder.IslandDailyOrder.RejectCount`
    #[serde(rename = "RejectCount", default)]
    pub reject_count: Option<String>,
    /// 旧键 `IslandDailyOrder.IslandDailyOrder.RejectFilter`
    #[serde(rename = "RejectFilter", default)]
    pub reject_filter: Option<String>,
    /// 旧键 `IslandDailyOrder.IslandDailyOrder.UrgentDetectRefreshTime`
    #[serde(rename = "UrgentDetectRefreshTime", default)]
    pub urgent_detect_refresh_time: Option<String>,
}

/// `IslandDailyOrder.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyOrderScheduler {
    /// 旧键 `IslandDailyOrder.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandDailyOrder.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandDailyOrder.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandDailyOrder.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandDailyOrder` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandDailyOrder {
    /// 旧键 `IslandDailyOrder.IslandDailyOrder.*`
    #[serde(rename = "IslandDailyOrder", default)]
    pub island_daily_order: Option<IslandDailyOrderIslandDailyOrder>,
    /// 旧键 `IslandDailyOrder.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandDailyOrderScheduler>,
}

/// `IslandFarm.IslandFarm` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandFarmIslandFarm {
    /// 旧键 `IslandFarm.IslandFarm.MinFarm`
    #[serde(rename = "MinFarm", default)]
    pub min_farm: Option<String>,
    /// 旧键 `IslandFarm.IslandFarm.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandFarm.IslandFarm.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
    /// 旧键 `IslandFarm.IslandFarm.PlantPotatoes`
    #[serde(rename = "PlantPotatoes", default)]
    pub plant_potatoes: Option<String>,
}

/// `IslandFarm.IslandNursery` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandFarmIslandNursery {
    /// 旧键 `IslandFarm.IslandNursery.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandFarm.IslandNursery.MinNursery`
    #[serde(rename = "MinNursery", default)]
    pub min_nursery: Option<String>,
    /// 旧键 `IslandFarm.IslandNursery.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
    /// 旧键 `IslandFarm.IslandNursery.PlantLavender`
    #[serde(rename = "PlantLavender", default)]
    pub plant_lavender: Option<String>,
    /// 旧键 `IslandFarm.IslandNursery.IgnorePineapple`
    #[serde(rename = "IgnorePineapple", default)]
    pub ignore_pineapple: Option<bool>,
}

/// `IslandFarm.IslandOrchard` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandFarmIslandOrchard {
    /// 旧键 `IslandFarm.IslandOrchard.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandFarm.IslandOrchard.MinOrchard`
    #[serde(rename = "MinOrchard", default)]
    pub min_orchard: Option<String>,
    /// 旧键 `IslandFarm.IslandOrchard.PlantRubber`
    #[serde(rename = "PlantRubber", default)]
    pub plant_rubber: Option<String>,
    /// 旧键 `IslandFarm.IslandOrchard.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
    /// 旧键 `IslandFarm.IslandOrchard.IgnoreAvocado`
    #[serde(rename = "IgnoreAvocado", default)]
    pub ignore_avocado: Option<bool>,
    /// 旧键 `IslandFarm.IslandOrchard.AmagiChanRubber`
    #[serde(rename = "AmagiChanRubber", default)]
    pub amagi_chan_rubber: Option<bool>,
}

/// `IslandFarm.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandFarmScheduler {
    /// 旧键 `IslandFarm.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandFarm.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandFarm.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandFarm.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandFarm` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandFarm {
    /// 旧键 `IslandFarm.IslandFarm.*`
    #[serde(rename = "IslandFarm", default)]
    pub island_farm: Option<IslandFarmIslandFarm>,
    /// 旧键 `IslandFarm.IslandNursery.*`
    #[serde(rename = "IslandNursery", default)]
    pub island_nursery: Option<IslandFarmIslandNursery>,
    /// 旧键 `IslandFarm.IslandOrchard.*`
    #[serde(rename = "IslandOrchard", default)]
    pub island_orchard: Option<IslandFarmIslandOrchard>,
    /// 旧键 `IslandFarm.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandFarmScheduler>,
}

/// `IslandGrill.IslandGrill` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandGrillIslandGrill {
    /// 旧键 `IslandGrill.IslandGrill.Meal1`
    #[serde(rename = "Meal1", default)]
    pub meal1: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal2`
    #[serde(rename = "Meal2", default)]
    pub meal2: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal3`
    #[serde(rename = "Meal3", default)]
    pub meal3: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal4`
    #[serde(rename = "Meal4", default)]
    pub meal4: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal5`
    #[serde(rename = "Meal5", default)]
    pub meal5: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal6`
    #[serde(rename = "Meal6", default)]
    pub meal6: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal7`
    #[serde(rename = "Meal7", default)]
    pub meal7: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.Meal8`
    #[serde(rename = "Meal8", default)]
    pub meal8: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.ChefFilter`
    #[serde(rename = "ChefFilter", default)]
    pub chef_filter: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.PostNumber`
    #[serde(rename = "PostNumber", default)]
    pub post_number: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber1`
    #[serde(rename = "MealNumber1", default)]
    pub meal_number1: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber2`
    #[serde(rename = "MealNumber2", default)]
    pub meal_number2: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber3`
    #[serde(rename = "MealNumber3", default)]
    pub meal_number3: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber4`
    #[serde(rename = "MealNumber4", default)]
    pub meal_number4: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber5`
    #[serde(rename = "MealNumber5", default)]
    pub meal_number5: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber6`
    #[serde(rename = "MealNumber6", default)]
    pub meal_number6: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber7`
    #[serde(rename = "MealNumber7", default)]
    pub meal_number7: Option<String>,
    /// 旧键 `IslandGrill.IslandGrill.MealNumber8`
    #[serde(rename = "MealNumber8", default)]
    pub meal_number8: Option<String>,
}

/// `IslandGrill.IslandGrillNextTask` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandGrillIslandGrillNextTask {
    /// 旧键 `IslandGrill.IslandGrillNextTask.AwayCook`
    #[serde(rename = "AwayCook", default)]
    pub away_cook: Option<String>,
}

/// `IslandGrill.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandGrillScheduler {
    /// 旧键 `IslandGrill.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandGrill.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandGrill.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandGrill.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandGrill` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandGrill {
    /// 旧键 `IslandGrill.IslandGrill.*`
    #[serde(rename = "IslandGrill", default)]
    pub island_grill: Option<IslandGrillIslandGrill>,
    /// 旧键 `IslandGrill.IslandGrillNextTask.*`
    #[serde(rename = "IslandGrillNextTask", default)]
    pub island_grill_next_task: Option<IslandGrillIslandGrillNextTask>,
    /// 旧键 `IslandGrill.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandGrillScheduler>,
}

/// `IslandJuuCoffee.IslandJuuCoffee` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuCoffeeIslandJuuCoffee {
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal1`
    #[serde(rename = "Meal1", default)]
    pub meal1: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal2`
    #[serde(rename = "Meal2", default)]
    pub meal2: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal3`
    #[serde(rename = "Meal3", default)]
    pub meal3: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal4`
    #[serde(rename = "Meal4", default)]
    pub meal4: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal5`
    #[serde(rename = "Meal5", default)]
    pub meal5: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal6`
    #[serde(rename = "Meal6", default)]
    pub meal6: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal7`
    #[serde(rename = "Meal7", default)]
    pub meal7: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Meal8`
    #[serde(rename = "Meal8", default)]
    pub meal8: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.Friedrich`
    #[serde(rename = "Friedrich", default)]
    pub friedrich: Option<bool>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.ChefFilter`
    #[serde(rename = "ChefFilter", default)]
    pub chef_filter: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.PostNumber`
    #[serde(rename = "PostNumber", default)]
    pub post_number: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber1`
    #[serde(rename = "MealNumber1", default)]
    pub meal_number1: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber2`
    #[serde(rename = "MealNumber2", default)]
    pub meal_number2: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber3`
    #[serde(rename = "MealNumber3", default)]
    pub meal_number3: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber4`
    #[serde(rename = "MealNumber4", default)]
    pub meal_number4: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber5`
    #[serde(rename = "MealNumber5", default)]
    pub meal_number5: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber6`
    #[serde(rename = "MealNumber6", default)]
    pub meal_number6: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber7`
    #[serde(rename = "MealNumber7", default)]
    pub meal_number7: Option<String>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.MealNumber8`
    #[serde(rename = "MealNumber8", default)]
    pub meal_number8: Option<String>,
}

/// `IslandJuuCoffee.IslandJuuCoffeeNextTask` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuCoffeeIslandJuuCoffeeNextTask {
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffeeNextTask.AwayCook`
    #[serde(rename = "AwayCook", default)]
    pub away_cook: Option<String>,
}

/// `IslandJuuCoffee.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuCoffeeScheduler {
    /// 旧键 `IslandJuuCoffee.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandJuuCoffee.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandJuuCoffee.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandJuuCoffee.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandJuuCoffee` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuCoffee {
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffee.*`
    #[serde(rename = "IslandJuuCoffee", default)]
    pub island_juu_coffee: Option<IslandJuuCoffeeIslandJuuCoffee>,
    /// 旧键 `IslandJuuCoffee.IslandJuuCoffeeNextTask.*`
    #[serde(rename = "IslandJuuCoffeeNextTask", default)]
    pub island_juu_coffee_next_task: Option<IslandJuuCoffeeIslandJuuCoffeeNextTask>,
    /// 旧键 `IslandJuuCoffee.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandJuuCoffeeScheduler>,
}

/// `IslandJuuEatery.IslandJuuEatery` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuEateryIslandJuuEatery {
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal1`
    #[serde(rename = "Meal1", default)]
    pub meal1: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal2`
    #[serde(rename = "Meal2", default)]
    pub meal2: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal3`
    #[serde(rename = "Meal3", default)]
    pub meal3: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal4`
    #[serde(rename = "Meal4", default)]
    pub meal4: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal5`
    #[serde(rename = "Meal5", default)]
    pub meal5: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal6`
    #[serde(rename = "Meal6", default)]
    pub meal6: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal7`
    #[serde(rename = "Meal7", default)]
    pub meal7: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.Meal8`
    #[serde(rename = "Meal8", default)]
    pub meal8: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.ChefFilter`
    #[serde(rename = "ChefFilter", default)]
    pub chef_filter: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.PostNumber`
    #[serde(rename = "PostNumber", default)]
    pub post_number: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber1`
    #[serde(rename = "MealNumber1", default)]
    pub meal_number1: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber2`
    #[serde(rename = "MealNumber2", default)]
    pub meal_number2: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber3`
    #[serde(rename = "MealNumber3", default)]
    pub meal_number3: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber4`
    #[serde(rename = "MealNumber4", default)]
    pub meal_number4: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber5`
    #[serde(rename = "MealNumber5", default)]
    pub meal_number5: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber6`
    #[serde(rename = "MealNumber6", default)]
    pub meal_number6: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber7`
    #[serde(rename = "MealNumber7", default)]
    pub meal_number7: Option<String>,
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.MealNumber8`
    #[serde(rename = "MealNumber8", default)]
    pub meal_number8: Option<String>,
}

/// `IslandJuuEatery.IslandJuuEateryNextTask` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuEateryIslandJuuEateryNextTask {
    /// 旧键 `IslandJuuEatery.IslandJuuEateryNextTask.AwayCook`
    #[serde(rename = "AwayCook", default)]
    pub away_cook: Option<String>,
}

/// `IslandJuuEatery.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuEateryScheduler {
    /// 旧键 `IslandJuuEatery.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandJuuEatery.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandJuuEatery.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandJuuEatery.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandJuuEatery` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandJuuEatery {
    /// 旧键 `IslandJuuEatery.IslandJuuEatery.*`
    #[serde(rename = "IslandJuuEatery", default)]
    pub island_juu_eatery: Option<IslandJuuEateryIslandJuuEatery>,
    /// 旧键 `IslandJuuEatery.IslandJuuEateryNextTask.*`
    #[serde(rename = "IslandJuuEateryNextTask", default)]
    pub island_juu_eatery_next_task: Option<IslandJuuEateryIslandJuuEateryNextTask>,
    /// 旧键 `IslandJuuEatery.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandJuuEateryScheduler>,
}

/// `IslandManufacture.ElectronicProcessing` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufactureElectronicProcessing {
    /// 旧键 `IslandManufacture.ElectronicProcessing.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
}

/// `IslandManufacture.Handmade` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufactureHandmade {
    /// 旧键 `IslandManufacture.Handmade.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
}

/// `IslandManufacture.Industrial` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufactureIndustrial {
    /// 旧键 `IslandManufacture.Industrial.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
}

/// `IslandManufacture.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufactureScheduler {
    /// 旧键 `IslandManufacture.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandManufacture.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandManufacture.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandManufacture.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandManufacture.WoodProcessing` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufactureWoodProcessing {
    /// 旧键 `IslandManufacture.WoodProcessing.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
}

/// `IslandManufacture` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandManufacture {
    /// 旧键 `IslandManufacture.ElectronicProcessing.*`
    #[serde(rename = "ElectronicProcessing", default)]
    pub electronic_processing: Option<IslandManufactureElectronicProcessing>,
    /// 旧键 `IslandManufacture.Handmade.*`
    #[serde(rename = "Handmade", default)]
    pub handmade: Option<IslandManufactureHandmade>,
    /// 旧键 `IslandManufacture.Industrial.*`
    #[serde(rename = "Industrial", default)]
    pub industrial: Option<IslandManufactureIndustrial>,
    /// 旧键 `IslandManufacture.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandManufactureScheduler>,
    /// 旧键 `IslandManufacture.WoodProcessing.*`
    #[serde(rename = "WoodProcessing", default)]
    pub wood_processing: Option<IslandManufactureWoodProcessing>,
}

/// `IslandMineForest.IslandForest` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandMineForestIslandForest {
    /// 旧键 `IslandMineForest.IslandForest.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandMineForest.IslandForest.CutElegant`
    #[serde(rename = "CutElegant", default)]
    pub cut_elegant: Option<String>,
    /// 旧键 `IslandMineForest.IslandForest.MinElegant`
    #[serde(rename = "MinElegant", default)]
    pub min_elegant: Option<String>,
    /// 旧键 `IslandMineForest.IslandForest.MinSelected`
    #[serde(rename = "MinSelected", default)]
    pub min_selected: Option<String>,
    /// 旧键 `IslandMineForest.IslandForest.MinPractical`
    #[serde(rename = "MinPractical", default)]
    pub min_practical: Option<String>,
    /// 旧键 `IslandMineForest.IslandForest.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
}

/// `IslandMineForest.IslandMine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandMineForestIslandMine {
    /// 旧键 `IslandMineForest.IslandMine.MinIron`
    #[serde(rename = "MinIron", default)]
    pub min_iron: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.MinCopper`
    #[serde(rename = "MinCopper", default)]
    pub min_copper: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.MinSilver`
    #[serde(rename = "MinSilver", default)]
    pub min_silver: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.MinSulphur`
    #[serde(rename = "MinSulphur", default)]
    pub min_sulphur: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.MineSilver`
    #[serde(rename = "MineSilver", default)]
    pub mine_silver: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.MinAluminium`
    #[serde(rename = "MinAluminium", default)]
    pub min_aluminium: Option<String>,
    /// 旧键 `IslandMineForest.IslandMine.WorkerFilter`
    #[serde(rename = "WorkerFilter", default)]
    pub worker_filter: Option<String>,
}

/// `IslandMineForest.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandMineForestScheduler {
    /// 旧键 `IslandMineForest.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandMineForest.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandMineForest.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandMineForest.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandMineForest` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandMineForest {
    /// 旧键 `IslandMineForest.IslandForest.*`
    #[serde(rename = "IslandForest", default)]
    pub island_forest: Option<IslandMineForestIslandForest>,
    /// 旧键 `IslandMineForest.IslandMine.*`
    #[serde(rename = "IslandMine", default)]
    pub island_mine: Option<IslandMineForestIslandMine>,
    /// 旧键 `IslandMineForest.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandMineForestScheduler>,
}

/// `IslandPearlSell.IslandPearlSell` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandPearlSellIslandPearlSell {
    /// 旧键 `IslandPearlSell.IslandPearlSell.BuyPrice`
    #[serde(rename = "BuyPrice", default)]
    pub buy_price: Option<String>,
    /// 旧键 `IslandPearlSell.IslandPearlSell.SellPrice`
    #[serde(rename = "SellPrice", default)]
    pub sell_price: Option<String>,
    /// 旧键 `IslandPearlSell.IslandPearlSell.BuyNextRun`
    #[serde(rename = "BuyNextRun", default)]
    pub buy_next_run: Option<String>,
    /// 旧键 `IslandPearlSell.IslandPearlSell.DailyPriceRefresh`
    #[serde(rename = "DailyPriceRefresh", default)]
    pub daily_price_refresh: Option<bool>,
    /// 旧键 `IslandPearlSell.IslandPearlSell.NextPearlTradeTime`
    #[serde(rename = "NextPearlTradeTime", default)]
    pub next_pearl_trade_time: Option<String>,
}

/// `IslandPearlSell.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandPearlSellScheduler {
    /// 旧键 `IslandPearlSell.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandPearlSell.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandPearlSell.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandPearlSell.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandPearlSell` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandPearlSell {
    /// 旧键 `IslandPearlSell.IslandPearlSell.*`
    #[serde(rename = "IslandPearlSell", default)]
    pub island_pearl_sell: Option<IslandPearlSellIslandPearlSell>,
    /// 旧键 `IslandPearlSell.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandPearlSellScheduler>,
}

/// `IslandPlan.IslandPlan` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandPlanIslandPlan {
    /// 旧键 `IslandPlan.IslandPlan.Season`
    #[serde(rename = "Season", default)]
    pub season: Option<String>,
}

/// `IslandPlan` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandPlan {
    /// 旧键 `IslandPlan.IslandPlan.*`
    #[serde(rename = "IslandPlan", default)]
    pub island_plan: Option<IslandPlanIslandPlan>,
}

/// `IslandRancher.IslandFishery` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRancherIslandFishery {
    /// 旧键 `IslandRancher.IslandFishery.MinBass`
    #[serde(rename = "MinBass", default)]
    pub min_bass: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinCrab`
    #[serde(rename = "MinCrab", default)]
    pub min_crab: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinShell`
    #[serde(rename = "MinShell", default)]
    pub min_shell: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinSquid`
    #[serde(rename = "MinSquid", default)]
    pub min_squid: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinShrimp`
    #[serde(rename = "MinShrimp", default)]
    pub min_shrimp: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.Positions`
    #[serde(rename = "Positions", default)]
    pub positions: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinCrayfish`
    #[serde(rename = "MinCrayfish", default)]
    pub min_crayfish: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.RancherFilter`
    #[serde(rename = "RancherFilter", default)]
    pub rancher_filter: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinSeaCucumber`
    #[serde(rename = "MinSeaCucumber", default)]
    pub min_sea_cucumber: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.MinYellowfinTuna`
    #[serde(rename = "MinYellowfinTuna", default)]
    pub min_yellowfin_tuna: Option<String>,
    /// 旧键 `IslandRancher.IslandFishery.PlantYellowfinTuna`
    #[serde(rename = "PlantYellowfinTuna", default)]
    pub plant_yellowfin_tuna: Option<String>,
}

/// `IslandRancher.IslandRancher` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRancherIslandRancher {
    /// 旧键 `IslandRancher.IslandRancher.Milk`
    #[serde(rename = "Milk", default)]
    pub milk: Option<bool>,
    /// 旧键 `IslandRancher.IslandRancher.Wool`
    #[serde(rename = "Wool", default)]
    pub wool: Option<bool>,
    /// 旧键 `IslandRancher.IslandRancher.MinPork`
    #[serde(rename = "MinPork", default)]
    pub min_pork: Option<String>,
    /// 旧键 `IslandRancher.IslandRancher.PigFilter`
    #[serde(rename = "PigFilter", default)]
    pub pig_filter: Option<String>,
    /// 旧键 `IslandRancher.IslandRancher.MinChicken`
    #[serde(rename = "MinChicken", default)]
    pub min_chicken: Option<String>,
    /// 旧键 `IslandRancher.IslandRancher.ChickenFilter`
    #[serde(rename = "ChickenFilter", default)]
    pub chicken_filter: Option<String>,
    /// 旧键 `IslandRancher.IslandRancher.RancherFilter`
    #[serde(rename = "RancherFilter", default)]
    pub rancher_filter: Option<String>,
    /// 旧键 `IslandRancher.IslandRancher.WoolWorkerFilter`
    #[serde(rename = "WoolWorkerFilter", default)]
    pub wool_worker_filter: Option<String>,
}

/// `IslandRancher.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRancherScheduler {
    /// 旧键 `IslandRancher.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandRancher.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandRancher.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandRancher.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandRancher` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRancher {
    /// 旧键 `IslandRancher.IslandFishery.*`
    #[serde(rename = "IslandFishery", default)]
    pub island_fishery: Option<IslandRancherIslandFishery>,
    /// 旧键 `IslandRancher.IslandRancher.*`
    #[serde(rename = "IslandRancher", default)]
    pub island_rancher: Option<IslandRancherIslandRancher>,
    /// 旧键 `IslandRancher.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandRancherScheduler>,
}

/// `IslandRestaurant.IslandRestaurant` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRestaurantIslandRestaurant {
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal1`
    #[serde(rename = "Meal1", default)]
    pub meal1: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal2`
    #[serde(rename = "Meal2", default)]
    pub meal2: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal3`
    #[serde(rename = "Meal3", default)]
    pub meal3: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal4`
    #[serde(rename = "Meal4", default)]
    pub meal4: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal5`
    #[serde(rename = "Meal5", default)]
    pub meal5: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal6`
    #[serde(rename = "Meal6", default)]
    pub meal6: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal7`
    #[serde(rename = "Meal7", default)]
    pub meal7: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.Meal8`
    #[serde(rename = "Meal8", default)]
    pub meal8: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.ChefFilter`
    #[serde(rename = "ChefFilter", default)]
    pub chef_filter: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.PostNumber`
    #[serde(rename = "PostNumber", default)]
    pub post_number: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber1`
    #[serde(rename = "MealNumber1", default)]
    pub meal_number1: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber2`
    #[serde(rename = "MealNumber2", default)]
    pub meal_number2: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber3`
    #[serde(rename = "MealNumber3", default)]
    pub meal_number3: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber4`
    #[serde(rename = "MealNumber4", default)]
    pub meal_number4: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber5`
    #[serde(rename = "MealNumber5", default)]
    pub meal_number5: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber6`
    #[serde(rename = "MealNumber6", default)]
    pub meal_number6: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber7`
    #[serde(rename = "MealNumber7", default)]
    pub meal_number7: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.MealNumber8`
    #[serde(rename = "MealNumber8", default)]
    pub meal_number8: Option<String>,
    /// 旧键 `IslandRestaurant.IslandRestaurant.DoubleBambooShoots`
    #[serde(rename = "DoubleBambooShoots", default)]
    pub double_bamboo_shoots: Option<bool>,
}

/// `IslandRestaurant.IslandRestaurantNextTask` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRestaurantIslandRestaurantNextTask {
    /// 旧键 `IslandRestaurant.IslandRestaurantNextTask.AwayCook`
    #[serde(rename = "AwayCook", default)]
    pub away_cook: Option<String>,
}

/// `IslandRestaurant.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRestaurantScheduler {
    /// 旧键 `IslandRestaurant.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandRestaurant.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandRestaurant.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandRestaurant.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandRestaurant` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandRestaurant {
    /// 旧键 `IslandRestaurant.IslandRestaurant.*`
    #[serde(rename = "IslandRestaurant", default)]
    pub island_restaurant: Option<IslandRestaurantIslandRestaurant>,
    /// 旧键 `IslandRestaurant.IslandRestaurantNextTask.*`
    #[serde(rename = "IslandRestaurantNextTask", default)]
    pub island_restaurant_next_task: Option<IslandRestaurantIslandRestaurantNextTask>,
    /// 旧键 `IslandRestaurant.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandRestaurantScheduler>,
}

/// `IslandTeahouse.IslandTeahouse` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandTeahouseIslandTeahouse {
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal1`
    #[serde(rename = "Meal1", default)]
    pub meal1: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal2`
    #[serde(rename = "Meal2", default)]
    pub meal2: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal3`
    #[serde(rename = "Meal3", default)]
    pub meal3: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal4`
    #[serde(rename = "Meal4", default)]
    pub meal4: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal5`
    #[serde(rename = "Meal5", default)]
    pub meal5: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal6`
    #[serde(rename = "Meal6", default)]
    pub meal6: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal7`
    #[serde(rename = "Meal7", default)]
    pub meal7: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Meal8`
    #[serde(rename = "Meal8", default)]
    pub meal8: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.Seasonal`
    #[serde(rename = "Seasonal", default)]
    pub seasonal: Option<bool>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.ChefFilter`
    #[serde(rename = "ChefFilter", default)]
    pub chef_filter: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.PostNumber`
    #[serde(rename = "PostNumber", default)]
    pub post_number: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber1`
    #[serde(rename = "MealNumber1", default)]
    pub meal_number1: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber2`
    #[serde(rename = "MealNumber2", default)]
    pub meal_number2: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber3`
    #[serde(rename = "MealNumber3", default)]
    pub meal_number3: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber4`
    #[serde(rename = "MealNumber4", default)]
    pub meal_number4: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber5`
    #[serde(rename = "MealNumber5", default)]
    pub meal_number5: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber6`
    #[serde(rename = "MealNumber6", default)]
    pub meal_number6: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber7`
    #[serde(rename = "MealNumber7", default)]
    pub meal_number7: Option<String>,
    /// 旧键 `IslandTeahouse.IslandTeahouse.MealNumber8`
    #[serde(rename = "MealNumber8", default)]
    pub meal_number8: Option<String>,
}

/// `IslandTeahouse.IslandTeahouseNextTask` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandTeahouseIslandTeahouseNextTask {
    /// 旧键 `IslandTeahouse.IslandTeahouseNextTask.AwayCook`
    #[serde(rename = "AwayCook", default)]
    pub away_cook: Option<String>,
}

/// `IslandTeahouse.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandTeahouseScheduler {
    /// 旧键 `IslandTeahouse.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `IslandTeahouse.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `IslandTeahouse.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `IslandTeahouse.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `IslandTeahouse` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct IslandTeahouse {
    /// 旧键 `IslandTeahouse.IslandTeahouse.*`
    #[serde(rename = "IslandTeahouse", default)]
    pub island_teahouse: Option<IslandTeahouseIslandTeahouse>,
    /// 旧键 `IslandTeahouse.IslandTeahouseNextTask.*`
    #[serde(rename = "IslandTeahouseNextTask", default)]
    pub island_teahouse_next_task: Option<IslandTeahouseIslandTeahouseNextTask>,
    /// 旧键 `IslandTeahouse.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<IslandTeahouseScheduler>,
}

/// `Main.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainCampaign {
    /// 旧键 `Main.Campaign.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Main.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Main.Campaign.AmbushEvade`
    #[serde(rename = "AmbushEvade", default)]
    pub ambush_evade: Option<bool>,
    /// 旧键 `Main.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Main.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Main.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Main.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Main.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Main.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainEmotion {
    /// 旧键 `Main.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Main.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Main.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Main.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Main.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Main.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Main.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Main.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Main.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Main.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Main.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Main.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Main.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainEnemyPriority {
    /// 旧键 `Main.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Main.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainFleet {
    /// 旧键 `Main.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Main.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Main.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Main.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Main.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Main.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Main.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Main.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Main.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Main.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Main.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainHpControl {
    /// 旧键 `Main.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Main.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Main.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Main.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Main.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Main.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Main.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Main.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Main.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainScheduler {
    /// 旧键 `Main.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Main.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Main.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Main.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Main.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainStopCondition {
    /// 旧键 `Main.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Main.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Main.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Main.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Main.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Main.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Main.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Main.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MainSubmarine {
    /// 旧键 `Main.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Main.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Main.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Main` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main {
    /// 旧键 `Main.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<MainCampaign>,
    /// 旧键 `Main.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<MainEmotion>,
    /// 旧键 `Main.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<MainEnemyPriority>,
    /// 旧键 `Main.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<MainFleet>,
    /// 旧键 `Main.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<MainHpControl>,
    /// 旧键 `Main.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<MainScheduler>,
    /// 旧键 `Main.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<MainStopCondition>,
    /// 旧键 `Main.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<MainSubmarine>,
}

/// `Main2.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2Campaign {
    /// 旧键 `Main2.Campaign.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main2.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Main2.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Main2.Campaign.AmbushEvade`
    #[serde(rename = "AmbushEvade", default)]
    pub ambush_evade: Option<bool>,
    /// 旧键 `Main2.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Main2.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Main2.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Main2.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Main2.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Main2.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2Emotion {
    /// 旧键 `Main2.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Main2.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Main2.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Main2.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Main2.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Main2.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Main2.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2EnemyPriority {
    /// 旧键 `Main2.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Main2.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2Fleet {
    /// 旧键 `Main2.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Main2.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Main2.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Main2.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Main2.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2HpControl {
    /// 旧键 `Main2.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Main2.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Main2.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Main2.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Main2.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Main2.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Main2.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Main2.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Main2.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2Scheduler {
    /// 旧键 `Main2.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Main2.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Main2.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Main2.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Main2.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2StopCondition {
    /// 旧键 `Main2.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Main2.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Main2.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Main2.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Main2.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Main2.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Main2.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Main2.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2Submarine {
    /// 旧键 `Main2.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main2.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Main2.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Main2.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Main2` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main2 {
    /// 旧键 `Main2.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<Main2Campaign>,
    /// 旧键 `Main2.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<Main2Emotion>,
    /// 旧键 `Main2.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<Main2EnemyPriority>,
    /// 旧键 `Main2.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<Main2Fleet>,
    /// 旧键 `Main2.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<Main2HpControl>,
    /// 旧键 `Main2.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<Main2Scheduler>,
    /// 旧键 `Main2.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<Main2StopCondition>,
    /// 旧键 `Main2.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<Main2Submarine>,
}

/// `Main3.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3Campaign {
    /// 旧键 `Main3.Campaign.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main3.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `Main3.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `Main3.Campaign.AmbushEvade`
    #[serde(rename = "AmbushEvade", default)]
    pub ambush_evade: Option<bool>,
    /// 旧键 `Main3.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `Main3.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `Main3.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `Main3.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Main3.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Main3.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3Emotion {
    /// 旧键 `Main3.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Main3.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `Main3.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Main3.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Main3.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `Main3.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `Main3.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3EnemyPriority {
    /// 旧键 `Main3.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `Main3.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3Fleet {
    /// 旧键 `Main3.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `Main3.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `Main3.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `Main3.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `Main3.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3HpControl {
    /// 旧键 `Main3.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `Main3.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `Main3.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `Main3.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `Main3.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `Main3.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `Main3.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `Main3.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `Main3.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3Scheduler {
    /// 旧键 `Main3.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Main3.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Main3.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Main3.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Main3.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3StopCondition {
    /// 旧键 `Main3.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Main3.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Main3.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `Main3.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `Main3.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `Main3.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `Main3.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `Main3.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3Submarine {
    /// 旧键 `Main3.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Main3.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `Main3.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `Main3.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `Main3` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Main3 {
    /// 旧键 `Main3.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<Main3Campaign>,
    /// 旧键 `Main3.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<Main3Emotion>,
    /// 旧键 `Main3.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<Main3EnemyPriority>,
    /// 旧键 `Main3.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<Main3Fleet>,
    /// 旧键 `Main3.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<Main3HpControl>,
    /// 旧键 `Main3.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<Main3Scheduler>,
    /// 旧键 `Main3.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<Main3StopCondition>,
    /// 旧键 `Main3.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<Main3Submarine>,
}

/// `MaritimeEscort.MaritimeEscort` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MaritimeEscortMaritimeEscort {
    /// 旧键 `MaritimeEscort.MaritimeEscort.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
}

/// `MaritimeEscort.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MaritimeEscortScheduler {
    /// 旧键 `MaritimeEscort.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `MaritimeEscort.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `MaritimeEscort.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `MaritimeEscort.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `MaritimeEscort` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MaritimeEscort {
    /// 旧键 `MaritimeEscort.MaritimeEscort.*`
    #[serde(rename = "MaritimeEscort", default)]
    pub maritime_escort: Option<MaritimeEscortMaritimeEscort>,
    /// 旧键 `MaritimeEscort.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<MaritimeEscortScheduler>,
}

/// `Meowfficer.Meowfficer` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MeowfficerMeowfficer {
    /// 旧键 `Meowfficer.Meowfficer.BuyAmount`
    #[serde(rename = "BuyAmount", default)]
    pub buy_amount: Option<String>,
    /// 旧键 `Meowfficer.Meowfficer.FortChoreMeowfficer`
    #[serde(rename = "FortChoreMeowfficer", default)]
    pub fort_chore_meowfficer: Option<bool>,
}

/// `Meowfficer.MeowfficerTrain` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MeowfficerMeowfficerTrain {
    /// 旧键 `Meowfficer.MeowfficerTrain.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Meowfficer.MeowfficerTrain.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Meowfficer.MeowfficerTrain.EnhanceIndex`
    #[serde(rename = "EnhanceIndex", default)]
    pub enhance_index: Option<String>,
    /// 旧键 `Meowfficer.MeowfficerTrain.MaxFeedLevel`
    #[serde(rename = "MaxFeedLevel", default)]
    pub max_feed_level: Option<String>,
    /// 旧键 `Meowfficer.MeowfficerTrain.RetainTalentedGold`
    #[serde(rename = "RetainTalentedGold", default)]
    pub retain_talented_gold: Option<bool>,
    /// 旧键 `Meowfficer.MeowfficerTrain.RetainTalentedPurple`
    #[serde(rename = "RetainTalentedPurple", default)]
    pub retain_talented_purple: Option<bool>,
}

/// `Meowfficer.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MeowfficerScheduler {
    /// 旧键 `Meowfficer.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Meowfficer.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Meowfficer.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Meowfficer.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Meowfficer` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Meowfficer {
    /// 旧键 `Meowfficer.Meowfficer.*`
    #[serde(rename = "Meowfficer", default)]
    pub meowfficer: Option<MeowfficerMeowfficer>,
    /// 旧键 `Meowfficer.MeowfficerTrain.*`
    #[serde(rename = "MeowfficerTrain", default)]
    pub meowfficer_train: Option<MeowfficerMeowfficerTrain>,
    /// 旧键 `Meowfficer.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<MeowfficerScheduler>,
}

/// `Minigame.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MinigameScheduler {
    /// 旧键 `Minigame.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Minigame.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Minigame.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Minigame.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Minigame` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Minigame {
    /// 旧键 `Minigame.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<MinigameScheduler>,
}

/// `OpsiAbyssal.OpsiAbyssal` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAbyssalOpsiAbyssal {
    /// 旧键 `OpsiAbyssal.OpsiAbyssal.ForceRun`
    #[serde(rename = "ForceRun", default)]
    pub force_run: Option<bool>,
}

/// `OpsiAbyssal.OpsiFleetFilter` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAbyssalOpsiFleetFilter {
    /// 旧键 `OpsiAbyssal.OpsiFleetFilter.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
}

/// `OpsiAbyssal.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAbyssalScheduler {
    /// 旧键 `OpsiAbyssal.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiAbyssal.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiAbyssal.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiAbyssal.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiAbyssal` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAbyssal {
    /// 旧键 `OpsiAbyssal.OpsiAbyssal.*`
    #[serde(rename = "OpsiAbyssal", default)]
    pub opsi_abyssal: Option<OpsiAbyssalOpsiAbyssal>,
    /// 旧键 `OpsiAbyssal.OpsiFleetFilter.*`
    #[serde(rename = "OpsiFleetFilter", default)]
    pub opsi_fleet_filter: Option<OpsiAbyssalOpsiFleetFilter>,
    /// 旧键 `OpsiAbyssal.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiAbyssalScheduler>,
}

/// `OpsiArchive.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiArchiveOpsiFleet {
    /// 旧键 `OpsiArchive.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `OpsiArchive.OpsiFleet.Submarine`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<bool>,
}

/// `OpsiArchive.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiArchiveScheduler {
    /// 旧键 `OpsiArchive.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiArchive.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiArchive.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiArchive.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiArchive` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiArchive {
    /// 旧键 `OpsiArchive.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiArchiveOpsiFleet>,
    /// 旧键 `OpsiArchive.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiArchiveScheduler>,
}

/// `OpsiAshAssist.OpsiAshAssist` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshAssistOpsiAshAssist {
    /// 旧键 `OpsiAshAssist.OpsiAshAssist.Tier`
    #[serde(rename = "Tier", default)]
    pub tier: Option<String>,
}

/// `OpsiAshAssist.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshAssistScheduler {
    /// 旧键 `OpsiAshAssist.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiAshAssist.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiAshAssist.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiAshAssist.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiAshAssist` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshAssist {
    /// 旧键 `OpsiAshAssist.OpsiAshAssist.*`
    #[serde(rename = "OpsiAshAssist", default)]
    pub opsi_ash_assist: Option<OpsiAshAssistOpsiAshAssist>,
    /// 旧键 `OpsiAshAssist.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiAshAssistScheduler>,
}

/// `OpsiAshBeacon.OpsiAshBeacon` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshBeaconOpsiAshBeacon {
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.AttackMode`
    #[serde(rename = "AttackMode", default)]
    pub attack_mode: Option<String>,
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.OneHitMode`
    #[serde(rename = "OneHitMode", default)]
    pub one_hit_mode: Option<bool>,
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.RequestAssist`
    #[serde(rename = "RequestAssist", default)]
    pub request_assist: Option<bool>,
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.AutoCollectShip`
    #[serde(rename = "AutoCollectShip", default)]
    pub auto_collect_ship: Option<bool>,
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.EnsureFullyCollected`
    #[serde(rename = "EnsureFullyCollected", default)]
    pub ensure_fully_collected: Option<bool>,
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.DossierAutoAttackMode`
    #[serde(rename = "DossierAutoAttackMode", default)]
    pub dossier_auto_attack_mode: Option<bool>,
}

/// `OpsiAshBeacon.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshBeaconScheduler {
    /// 旧键 `OpsiAshBeacon.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiAshBeacon.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiAshBeacon.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiAshBeacon.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiAshBeacon` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiAshBeacon {
    /// 旧键 `OpsiAshBeacon.OpsiAshBeacon.*`
    #[serde(rename = "OpsiAshBeacon", default)]
    pub opsi_ash_beacon: Option<OpsiAshBeaconOpsiAshBeacon>,
    /// 旧键 `OpsiAshBeacon.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiAshBeaconScheduler>,
}

/// `OpsiCrossMonth.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiCrossMonthScheduler {
    /// 旧键 `OpsiCrossMonth.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiCrossMonth.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiCrossMonth.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiCrossMonth.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiCrossMonth` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiCrossMonth {
    /// 旧键 `OpsiCrossMonth.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiCrossMonthScheduler>,
}

/// `OpsiDaemon.OpsiDaemon` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDaemonOpsiDaemon {
    /// 旧键 `OpsiDaemon.OpsiDaemon.RepairShip`
    #[serde(rename = "RepairShip", default)]
    pub repair_ship: Option<bool>,
    /// 旧键 `OpsiDaemon.OpsiDaemon.SelectEnemy`
    #[serde(rename = "SelectEnemy", default)]
    pub select_enemy: Option<bool>,
}

/// `OpsiDaemon` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDaemon {
    /// 旧键 `OpsiDaemon.OpsiDaemon.*`
    #[serde(rename = "OpsiDaemon", default)]
    pub opsi_daemon: Option<OpsiDaemonOpsiDaemon>,
}

/// `OpsiDaily.OpsiDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDailyOpsiDaily {
    /// 旧键 `OpsiDaily.OpsiDaily.DoMission`
    #[serde(rename = "DoMission", default)]
    pub do_mission: Option<bool>,
    /// 旧键 `OpsiDaily.OpsiDaily.KeepMissionZone`
    #[serde(rename = "KeepMissionZone", default)]
    pub keep_mission_zone: Option<bool>,
    /// 旧键 `OpsiDaily.OpsiDaily.UseTuningSample`
    #[serde(rename = "UseTuningSample", default)]
    pub use_tuning_sample: Option<bool>,
    /// 旧键 `OpsiDaily.OpsiDaily.CollectTargetReward`
    #[serde(rename = "CollectTargetReward", default)]
    pub collect_target_reward: Option<bool>,
    /// 旧键 `OpsiDaily.OpsiDaily.SkipSirenResearchMission`
    #[serde(rename = "SkipSirenResearchMission", default)]
    pub skip_siren_research_mission: Option<bool>,
}

/// `OpsiDaily.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDailyOpsiFleet {
    /// 旧键 `OpsiDaily.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `OpsiDaily.OpsiFleet.Submarine`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<bool>,
}

/// `OpsiDaily.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDailyScheduler {
    /// 旧键 `OpsiDaily.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiDaily.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiDaily.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiDaily.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiDaily` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiDaily {
    /// 旧键 `OpsiDaily.OpsiDaily.*`
    #[serde(rename = "OpsiDaily", default)]
    pub opsi_daily: Option<OpsiDailyOpsiDaily>,
    /// 旧键 `OpsiDaily.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiDailyOpsiFleet>,
    /// 旧键 `OpsiDaily.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiDailyScheduler>,
}

/// `OpsiExplore.OpsiExplore` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiExploreOpsiExplore {
    /// 旧键 `OpsiExplore.OpsiExplore.ForceRun`
    #[serde(rename = "ForceRun", default)]
    pub force_run: Option<bool>,
    /// 旧键 `OpsiExplore.OpsiExplore.LastZone`
    #[serde(rename = "LastZone", default)]
    pub last_zone: Option<String>,
    /// 旧键 `OpsiExplore.OpsiExplore.SpecialRadar`
    #[serde(rename = "SpecialRadar", default)]
    pub special_radar: Option<bool>,
    /// 旧键 `OpsiExplore.OpsiExplore.ExploreProgress`
    #[serde(rename = "ExploreProgress", default)]
    pub explore_progress: Option<String>,
    /// 旧键 `OpsiExplore.OpsiExplore.AllowHazard1Leveling`
    #[serde(rename = "AllowHazard1Leveling", default)]
    pub allow_hazard1_leveling: Option<bool>,
}

/// `OpsiExplore.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiExploreOpsiFleet {
    /// 旧键 `OpsiExplore.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `OpsiExplore.OpsiFleet.Submarine`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<bool>,
}

/// `OpsiExplore.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiExploreScheduler {
    /// 旧键 `OpsiExplore.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiExplore.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiExplore.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiExplore.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiExplore` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiExplore {
    /// 旧键 `OpsiExplore.OpsiExplore.*`
    #[serde(rename = "OpsiExplore", default)]
    pub opsi_explore: Option<OpsiExploreOpsiExplore>,
    /// 旧键 `OpsiExplore.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiExploreOpsiFleet>,
    /// 旧键 `OpsiExplore.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiExploreScheduler>,
}

/// `OpsiGeneral.OpsiGeneral` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiGeneralOpsiGeneral {
    /// 旧键 `OpsiGeneral.OpsiGeneral.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.UseLogger`
    #[serde(rename = "UseLogger", default)]
    pub use_logger: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.LauncherPush`
    #[serde(rename = "LauncherPush", default)]
    pub launcher_push: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.UseRepairPack`
    #[serde(rename = "UseRepairPack", default)]
    pub use_repair_pack: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.NotifyOpsiMail`
    #[serde(rename = "NotifyOpsiMail", default)]
    pub notify_opsi_mail: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.IndependentPush`
    #[serde(rename = "IndependentPush", default)]
    pub independent_push: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.AkashiShopFilter`
    #[serde(rename = "AkashiShopFilter", default)]
    pub akashi_shop_filter: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.DoRandomMapEvent`
    #[serde(rename = "DoRandomMapEvent", default)]
    pub do_random_map_event: Option<bool>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.OpsiOnePushConfig`
    #[serde(rename = "OpsiOnePushConfig", default)]
    pub opsi_one_push_config: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.AutoSearchTimeLimit`
    #[serde(rename = "AutoSearchTimeLimit", default)]
    pub auto_search_time_limit: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.BuyActionPointLimit`
    #[serde(rename = "BuyActionPointLimit", default)]
    pub buy_action_point_limit: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.RepairThreshold`
    #[serde(rename = "RepairThreshold", default)]
    pub repair_threshold: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.RepairPackThreshold`
    #[serde(rename = "RepairPackThreshold", default)]
    pub repair_pack_threshold: Option<String>,
    /// 旧键 `OpsiGeneral.OpsiGeneral.RepairPackThresholdHazard1`
    #[serde(rename = "RepairPackThresholdHazard1", default)]
    pub repair_pack_threshold_hazard1: Option<String>,
}

/// `OpsiGeneral` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiGeneral {
    /// 旧键 `OpsiGeneral.OpsiGeneral.*`
    #[serde(rename = "OpsiGeneral", default)]
    pub opsi_general: Option<OpsiGeneralOpsiGeneral>,
}

/// `OpsiHazard1Leveling.OpsiCheckLeveling` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingOpsiCheckLeveling {
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.TargetLevel`
    #[serde(rename = "TargetLevel", default)]
    pub target_level: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.DelayAfterFull`
    #[serde(rename = "DelayAfterFull", default)]
    pub delay_after_full: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.EnableCustomCheck`
    #[serde(rename = "EnableCustomCheck", default)]
    pub enable_custom_check: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.CustomCheckPositions`
    #[serde(rename = "CustomCheckPositions", default)]
    pub custom_check_positions: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.CheckInterval`
    #[serde(rename = "CheckInterval", default)]
    pub check_interval: Option<String>,
}

/// `OpsiHazard1Leveling.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingOpsiFleet {
    /// 旧键 `OpsiHazard1Leveling.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `OpsiHazard1Leveling.OpsiFleetAutoChange` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingOpsiFleetAutoChange {
    /// 旧键 `OpsiHazard1Leveling.OpsiFleetAutoChange.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiFleetAutoChange.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiFleetAutoChange.CooldownHours`
    #[serde(rename = "CooldownHours", default)]
    pub cooldown_hours: Option<String>,
}

/// `OpsiHazard1Leveling.OpsiHazard1Leveling` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingOpsiHazard1Leveling {
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.Cl1Filter`
    #[serde(rename = "Cl1Filter", default)]
    pub cl1_filter: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.TargetZone`
    #[serde(rename = "TargetZone", default)]
    pub target_zone: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.SkipHpCheck`
    #[serde(rename = "SkipHpCheck", default)]
    pub skip_hp_check: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.RecordNonCL1AP`
    #[serde(rename = "RecordNonCL1AP", default)]
    pub record_non_cl1_ap: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.RecordSeaMiles`
    #[serde(rename = "RecordSeaMiles", default)]
    pub record_sea_miles: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.ExecuteFixedPatrolScan`
    #[serde(rename = "ExecuteFixedPatrolScan", default)]
    pub execute_fixed_patrol_scan: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.OperationCoinsPreserve`
    #[serde(rename = "OperationCoinsPreserve", default)]
    pub operation_coins_preserve: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.MinimumActionPointReserve`
    #[serde(rename = "MinimumActionPointReserve", default)]
    pub minimum_action_point_reserve: Option<String>,
}

/// `OpsiHazard1Leveling.OpsiSirenBug` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingOpsiSirenBug {
    /// 旧键 `OpsiHazard1Leveling.OpsiSirenBug.Siren_Mode`
    #[serde(rename = "Siren_Mode", default)]
    pub siren_mode: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiSirenBug.Siren_Fleet`
    #[serde(rename = "Siren_Fleet", default)]
    pub siren_fleet: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.OpsiSirenBug.SirenResearch_Enable`
    #[serde(rename = "SirenResearch_Enable", default)]
    pub siren_research_enable: Option<bool>,
}

/// `OpsiHazard1Leveling.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1LevelingScheduler {
    /// 旧键 `OpsiHazard1Leveling.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiHazard1Leveling.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiHazard1Leveling.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiHazard1Leveling` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiHazard1Leveling {
    /// 旧键 `OpsiHazard1Leveling.OpsiCheckLeveling.*`
    #[serde(rename = "OpsiCheckLeveling", default)]
    pub opsi_check_leveling: Option<OpsiHazard1LevelingOpsiCheckLeveling>,
    /// 旧键 `OpsiHazard1Leveling.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiHazard1LevelingOpsiFleet>,
    /// 旧键 `OpsiHazard1Leveling.OpsiFleetAutoChange.*`
    #[serde(rename = "OpsiFleetAutoChange", default)]
    pub opsi_fleet_auto_change: Option<OpsiHazard1LevelingOpsiFleetAutoChange>,
    /// 旧键 `OpsiHazard1Leveling.OpsiHazard1Leveling.*`
    #[serde(rename = "OpsiHazard1Leveling", default)]
    pub opsi_hazard1_leveling: Option<OpsiHazard1LevelingOpsiHazard1Leveling>,
    /// 旧键 `OpsiHazard1Leveling.OpsiSirenBug.*`
    #[serde(rename = "OpsiSirenBug", default)]
    pub opsi_siren_bug: Option<OpsiHazard1LevelingOpsiSirenBug>,
    /// 旧键 `OpsiHazard1Leveling.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiHazard1LevelingScheduler>,
}

/// `OpsiMeowfficerFarming.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarmingOpsiFleet {
    /// 旧键 `OpsiMeowfficerFarming.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiFleet.Submarine`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<bool>,
}

/// `OpsiMeowfficerFarming.OpsiMeowfficerFarming` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarmingOpsiMeowfficerFarming {
    /// 旧键 `OpsiMeowfficerFarming.OpsiMeowfficerFarming.StayInZone`
    #[serde(rename = "StayInZone", default)]
    pub stay_in_zone: Option<bool>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiMeowfficerFarming.TargetZone`
    #[serde(rename = "TargetZone", default)]
    pub target_zone: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiMeowfficerFarming.HazardLevel`
    #[serde(rename = "HazardLevel", default)]
    pub hazard_level: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiMeowfficerFarming.ActionPointPreserve`
    #[serde(rename = "ActionPointPreserve", default)]
    pub action_point_preserve: Option<String>,
}

/// `OpsiMeowfficerFarming.OpsiSirenBug` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarmingOpsiSirenBug {
    /// 旧键 `OpsiMeowfficerFarming.OpsiSirenBug.Siren_Mode`
    #[serde(rename = "Siren_Mode", default)]
    pub siren_mode: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiSirenBug.Siren_Fleet`
    #[serde(rename = "Siren_Fleet", default)]
    pub siren_fleet: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiSirenBug.SirenResearch_Enable`
    #[serde(rename = "SirenResearch_Enable", default)]
    pub siren_research_enable: Option<bool>,
}

/// `OpsiMeowfficerFarming.OpsiTarget` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarmingOpsiTarget {
    /// 旧键 `OpsiMeowfficerFarming.OpsiTarget.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiTarget.TargetZone`
    #[serde(rename = "TargetZone", default)]
    pub target_zone: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiTarget.TargetFarming`
    #[serde(rename = "TargetFarming", default)]
    pub target_farming: Option<bool>,
}

/// `OpsiMeowfficerFarming.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarmingScheduler {
    /// 旧键 `OpsiMeowfficerFarming.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiMeowfficerFarming.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiMeowfficerFarming.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiMeowfficerFarming.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiMeowfficerFarming` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMeowfficerFarming {
    /// 旧键 `OpsiMeowfficerFarming.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiMeowfficerFarmingOpsiFleet>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiMeowfficerFarming.*`
    #[serde(rename = "OpsiMeowfficerFarming", default)]
    pub opsi_meowfficer_farming: Option<OpsiMeowfficerFarmingOpsiMeowfficerFarming>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiSirenBug.*`
    #[serde(rename = "OpsiSirenBug", default)]
    pub opsi_siren_bug: Option<OpsiMeowfficerFarmingOpsiSirenBug>,
    /// 旧键 `OpsiMeowfficerFarming.OpsiTarget.*`
    #[serde(rename = "OpsiTarget", default)]
    pub opsi_target: Option<OpsiMeowfficerFarmingOpsiTarget>,
    /// 旧键 `OpsiMeowfficerFarming.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiMeowfficerFarmingScheduler>,
}

/// `OpsiMonthBoss.OpsiFleetFilter` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMonthBossOpsiFleetFilter {
    /// 旧键 `OpsiMonthBoss.OpsiFleetFilter.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
}

/// `OpsiMonthBoss.OpsiMonthBoss` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMonthBossOpsiMonthBoss {
    /// 旧键 `OpsiMonthBoss.OpsiMonthBoss.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `OpsiMonthBoss.OpsiMonthBoss.ForceRun`
    #[serde(rename = "ForceRun", default)]
    pub force_run: Option<bool>,
    /// 旧键 `OpsiMonthBoss.OpsiMonthBoss.CheckAdaptability`
    #[serde(rename = "CheckAdaptability", default)]
    pub check_adaptability: Option<bool>,
}

/// `OpsiMonthBoss.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMonthBossScheduler {
    /// 旧键 `OpsiMonthBoss.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiMonthBoss.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiMonthBoss.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiMonthBoss.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiMonthBoss` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiMonthBoss {
    /// 旧键 `OpsiMonthBoss.OpsiFleetFilter.*`
    #[serde(rename = "OpsiFleetFilter", default)]
    pub opsi_fleet_filter: Option<OpsiMonthBossOpsiFleetFilter>,
    /// 旧键 `OpsiMonthBoss.OpsiMonthBoss.*`
    #[serde(rename = "OpsiMonthBoss", default)]
    pub opsi_month_boss: Option<OpsiMonthBossOpsiMonthBoss>,
    /// 旧键 `OpsiMonthBoss.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiMonthBossScheduler>,
}

/// `OpsiObscure.OpsiFleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiObscureOpsiFleet {
    /// 旧键 `OpsiObscure.OpsiFleet.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `OpsiObscure.OpsiFleet.Submarine`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<bool>,
}

/// `OpsiObscure.OpsiObscure` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiObscureOpsiObscure {
    /// 旧键 `OpsiObscure.OpsiObscure.ForceRun`
    #[serde(rename = "ForceRun", default)]
    pub force_run: Option<bool>,
    /// 旧键 `OpsiObscure.OpsiObscure.SkipHazard2Obscure`
    #[serde(rename = "SkipHazard2Obscure", default)]
    pub skip_hazard2_obscure: Option<bool>,
}

/// `OpsiObscure.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiObscureScheduler {
    /// 旧键 `OpsiObscure.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiObscure.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiObscure.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiObscure.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiObscure` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiObscure {
    /// 旧键 `OpsiObscure.OpsiFleet.*`
    #[serde(rename = "OpsiFleet", default)]
    pub opsi_fleet: Option<OpsiObscureOpsiFleet>,
    /// 旧键 `OpsiObscure.OpsiObscure.*`
    #[serde(rename = "OpsiObscure", default)]
    pub opsi_obscure: Option<OpsiObscureOpsiObscure>,
    /// 旧键 `OpsiObscure.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiObscureScheduler>,
}

/// `OpsiPreventActionPointOverflow.OpsiPreventActionPointOverflow` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiPreventActionPointOverflowOpsiPreventActionPointOverflow {
    /// 旧键 `OpsiPreventActionPointOverflow.OpsiPreventActionPointOverflow.Task`
    #[serde(rename = "Task", default)]
    pub task: Option<String>,
    /// 旧键 `OpsiPreventActionPointOverflow.OpsiPreventActionPointOverflow.ActionPointLowerbound`
    #[serde(rename = "ActionPointLowerbound", default)]
    pub action_point_lowerbound: Option<String>,
    /// 旧键 `OpsiPreventActionPointOverflow.OpsiPreventActionPointOverflow.ActionPointUpperbound`
    #[serde(rename = "ActionPointUpperbound", default)]
    pub action_point_upperbound: Option<String>,
}

/// `OpsiPreventActionPointOverflow.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiPreventActionPointOverflowScheduler {
    /// 旧键 `OpsiPreventActionPointOverflow.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiPreventActionPointOverflow.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiPreventActionPointOverflow.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiPreventActionPointOverflow.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiPreventActionPointOverflow` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiPreventActionPointOverflow {
    /// 旧键 `OpsiPreventActionPointOverflow.OpsiPreventActionPointOverflow.*`
    #[serde(rename = "OpsiPreventActionPointOverflow", default)]
    pub opsi_prevent_action_point_overflow: Option<OpsiPreventActionPointOverflowOpsiPreventActionPointOverflow>,
    /// 旧键 `OpsiPreventActionPointOverflow.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiPreventActionPointOverflowScheduler>,
}

/// `OpsiScheduling.OpsiScheduling` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiSchedulingOpsiScheduling {
    /// 旧键 `OpsiScheduling.OpsiScheduling.TaskPriority`
    #[serde(rename = "TaskPriority", default)]
    pub task_priority: Option<String>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.EnableAbyssal`
    #[serde(rename = "EnableAbyssal", default)]
    pub enable_abyssal: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.EnableObscure`
    #[serde(rename = "EnableObscure", default)]
    pub enable_obscure: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.EnableStronghold`
    #[serde(rename = "EnableStronghold", default)]
    pub enable_stronghold: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.ActionPointPreserve`
    #[serde(rename = "ActionPointPreserve", default)]
    pub action_point_preserve: Option<String>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.MonthEndShopPurchase`
    #[serde(rename = "MonthEndShopPurchase", default)]
    pub month_end_shop_purchase: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.OperationCoinsPreserve`
    #[serde(rename = "OperationCoinsPreserve", default)]
    pub operation_coins_preserve: Option<String>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.EnableMeowfficerFarming`
    #[serde(rename = "EnableMeowfficerFarming", default)]
    pub enable_meowfficer_farming: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.MonthEndActionPointPreserve`
    #[serde(rename = "MonthEndActionPointPreserve", default)]
    pub month_end_action_point_preserve: Option<String>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.MonthEndActionPointCleanupDays`
    #[serde(rename = "MonthEndActionPointCleanupDays", default)]
    pub month_end_action_point_cleanup_days: Option<String>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.MonthEndActionPointCleanupEnable`
    #[serde(rename = "MonthEndActionPointCleanupEnable", default)]
    pub month_end_action_point_cleanup_enable: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.UseSmartSchedulingOperationCoinsPreserve`
    #[serde(rename = "UseSmartSchedulingOperationCoinsPreserve", default)]
    pub use_smart_scheduling_operation_coins_preserve: Option<bool>,
    /// 旧键 `OpsiScheduling.OpsiScheduling.OperationCoinsReturnThreshold`
    #[serde(rename = "OperationCoinsReturnThreshold", default)]
    pub operation_coins_return_threshold: Option<String>,
}

/// `OpsiScheduling.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiSchedulingScheduler {
    /// 旧键 `OpsiScheduling.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiScheduling.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiScheduling.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiScheduling.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiScheduling` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiScheduling {
    /// 旧键 `OpsiScheduling.OpsiScheduling.*`
    #[serde(rename = "OpsiScheduling", default)]
    pub opsi_scheduling: Option<OpsiSchedulingOpsiScheduling>,
    /// 旧键 `OpsiScheduling.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiSchedulingScheduler>,
}

/// `OpsiShop.OpsiShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiShopOpsiShop {
    /// 旧键 `OpsiShop.OpsiShop.CustomFilter`
    #[serde(rename = "CustomFilter", default)]
    pub custom_filter: Option<String>,
    /// 旧键 `OpsiShop.OpsiShop.PresetFilter`
    #[serde(rename = "PresetFilter", default)]
    pub preset_filter: Option<String>,
    /// 旧键 `OpsiShop.OpsiShop.DisableBeforeDate`
    #[serde(rename = "DisableBeforeDate", default)]
    pub disable_before_date: Option<String>,
}

/// `OpsiShop.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiShopScheduler {
    /// 旧键 `OpsiShop.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiShop.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiShop.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiShop.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiShop` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiShop {
    /// 旧键 `OpsiShop.OpsiShop.*`
    #[serde(rename = "OpsiShop", default)]
    pub opsi_shop: Option<OpsiShopOpsiShop>,
    /// 旧键 `OpsiShop.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiShopScheduler>,
}

/// `OpsiSimulator.OpsiSimulatorParameters` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiSimulatorOpsiSimulatorParameters {
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Draw`
    #[serde(rename = "Draw", default)]
    pub draw: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.BuyAp`
    #[serde(rename = "BuyAp", default)]
    pub buy_ap: Option<bool>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Cl1Coin`
    #[serde(rename = "Cl1Coin", default)]
    pub cl1_coin: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Cl1Time`
    #[serde(rename = "Cl1Time", default)]
    pub cl1_time: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Samples`
    #[serde(rename = "Samples", default)]
    pub samples: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.CrossWeek`
    #[serde(rename = "CrossWeek", default)]
    pub cross_week: Option<bool>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.InitialAp`
    #[serde(rename = "InitialAp", default)]
    pub initial_ap: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Meow3Coin`
    #[serde(rename = "Meow3Coin", default)]
    pub meow3_coin: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Meow3Time`
    #[serde(rename = "Meow3Time", default)]
    pub meow3_time: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Meow5Coin`
    #[serde(rename = "Meow5Coin", default)]
    pub meow5_coin: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Meow5Time`
    #[serde(rename = "Meow5Time", default)]
    pub meow5_time: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.TotalTime`
    #[serde(rename = "TotalTime", default)]
    pub total_time: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.InitialCoin`
    #[serde(rename = "InitialCoin", default)]
    pub initial_coin: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.TimeUseRatio`
    #[serde(rename = "TimeUseRatio", default)]
    pub time_use_ratio: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.Deterministic`
    #[serde(rename = "Deterministic", default)]
    pub deterministic: Option<bool>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.MeowHazardLevel`
    #[serde(rename = "MeowHazardLevel", default)]
    pub meow_hazard_level: Option<String>,
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.AkashiProbability`
    #[serde(rename = "AkashiProbability", default)]
    pub akashi_probability: Option<String>,
}

/// `OpsiSimulator` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiSimulator {
    /// 旧键 `OpsiSimulator.OpsiSimulatorParameters.*`
    #[serde(rename = "OpsiSimulatorParameters", default)]
    pub opsi_simulator_parameters: Option<OpsiSimulatorOpsiSimulatorParameters>,
}

/// `OpsiStronghold.OpsiFleetFilter` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiStrongholdOpsiFleetFilter {
    /// 旧键 `OpsiStronghold.OpsiFleetFilter.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
}

/// `OpsiStronghold.OpsiStronghold` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiStrongholdOpsiStronghold {
    /// 旧键 `OpsiStronghold.OpsiStronghold.ForceRun`
    #[serde(rename = "ForceRun", default)]
    pub force_run: Option<bool>,
    /// 旧键 `OpsiStronghold.OpsiStronghold.SubmarineEveryCombat`
    #[serde(rename = "SubmarineEveryCombat", default)]
    pub submarine_every_combat: Option<bool>,
}

/// `OpsiStronghold.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiStrongholdScheduler {
    /// 旧键 `OpsiStronghold.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiStronghold.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiStronghold.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiStronghold.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiStronghold` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiStronghold {
    /// 旧键 `OpsiStronghold.OpsiFleetFilter.*`
    #[serde(rename = "OpsiFleetFilter", default)]
    pub opsi_fleet_filter: Option<OpsiStrongholdOpsiFleetFilter>,
    /// 旧键 `OpsiStronghold.OpsiStronghold.*`
    #[serde(rename = "OpsiStronghold", default)]
    pub opsi_stronghold: Option<OpsiStrongholdOpsiStronghold>,
    /// 旧键 `OpsiStronghold.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiStrongholdScheduler>,
}

/// `OpsiVoucher.OpsiVoucher` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiVoucherOpsiVoucher {
    /// 旧键 `OpsiVoucher.OpsiVoucher.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
}

/// `OpsiVoucher.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiVoucherScheduler {
    /// 旧键 `OpsiVoucher.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `OpsiVoucher.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `OpsiVoucher.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `OpsiVoucher.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `OpsiVoucher` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OpsiVoucher {
    /// 旧键 `OpsiVoucher.OpsiVoucher.*`
    #[serde(rename = "OpsiVoucher", default)]
    pub opsi_voucher: Option<OpsiVoucherOpsiVoucher>,
    /// 旧键 `OpsiVoucher.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<OpsiVoucherScheduler>,
}

/// `PrivateQuarters.PrivateQuarters` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct PrivateQuartersPrivateQuarters {
    /// 旧键 `PrivateQuarters.PrivateQuarters.BuyCake`
    #[serde(rename = "BuyCake", default)]
    pub buy_cake: Option<bool>,
    /// 旧键 `PrivateQuarters.PrivateQuarters.BuyRoses`
    #[serde(rename = "BuyRoses", default)]
    pub buy_roses: Option<bool>,
    /// 旧键 `PrivateQuarters.PrivateQuarters.TargetShip`
    #[serde(rename = "TargetShip", default)]
    pub target_ship: Option<String>,
    /// 旧键 `PrivateQuarters.PrivateQuarters.TargetInteract`
    #[serde(rename = "TargetInteract", default)]
    pub target_interact: Option<bool>,
}

/// `PrivateQuarters.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct PrivateQuartersScheduler {
    /// 旧键 `PrivateQuarters.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `PrivateQuarters.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `PrivateQuarters.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `PrivateQuarters.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `PrivateQuarters` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct PrivateQuarters {
    /// 旧键 `PrivateQuarters.PrivateQuarters.*`
    #[serde(rename = "PrivateQuarters", default)]
    pub private_quarters: Option<PrivateQuartersPrivateQuarters>,
    /// 旧键 `PrivateQuarters.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<PrivateQuartersScheduler>,
}

/// `Raid.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidCampaign {
    /// 旧键 `Raid.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `Raid.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `Raid.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `Raid.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidEmotion {
    /// 旧键 `Raid.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Raid.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `Raid.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `Raid.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `Raid.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `Raid.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `Raid.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
}

/// `Raid.Raid` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidRaid {
    /// 旧键 `Raid.Raid.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `Raid.Raid.UseTicket`
    #[serde(rename = "UseTicket", default)]
    pub use_ticket: Option<bool>,
}

/// `Raid.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScheduler {
    /// 旧键 `Raid.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Raid.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Raid.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Raid.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Raid.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidStopCondition {
    /// 旧键 `Raid.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `Raid.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `Raid.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `Raid` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Raid {
    /// 旧键 `Raid.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<RaidCampaign>,
    /// 旧键 `Raid.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<RaidEmotion>,
    /// 旧键 `Raid.Raid.*`
    #[serde(rename = "Raid", default)]
    pub raid: Option<RaidRaid>,
    /// 旧键 `Raid.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<RaidScheduler>,
    /// 旧键 `Raid.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<RaidStopCondition>,
}

/// `RaidDaily.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDailyCampaign {
    /// 旧键 `RaidDaily.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `RaidDaily.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `RaidDaily.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `RaidDaily.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDailyEmotion {
    /// 旧键 `RaidDaily.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `RaidDaily.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
}

/// `RaidDaily.RaidDaily` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDailyRaidDaily {
    /// 旧键 `RaidDaily.RaidDaily.StageFilter`
    #[serde(rename = "StageFilter", default)]
    pub stage_filter: Option<String>,
}

/// `RaidDaily.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDailyScheduler {
    /// 旧键 `RaidDaily.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `RaidDaily.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `RaidDaily.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `RaidDaily.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `RaidDaily.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDailyStopCondition {
    /// 旧键 `RaidDaily.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `RaidDaily.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `RaidDaily` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidDaily {
    /// 旧键 `RaidDaily.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<RaidDailyCampaign>,
    /// 旧键 `RaidDaily.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<RaidDailyEmotion>,
    /// 旧键 `RaidDaily.RaidDaily.*`
    #[serde(rename = "RaidDaily", default)]
    pub raid_daily: Option<RaidDailyRaidDaily>,
    /// 旧键 `RaidDaily.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<RaidDailyScheduler>,
    /// 旧键 `RaidDaily.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<RaidDailyStopCondition>,
}

/// `RaidScuttle.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleCampaign {
    /// 旧键 `RaidScuttle.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `RaidScuttle.Campaign.AmbushEvade`
    #[serde(rename = "AmbushEvade", default)]
    pub ambush_evade: Option<bool>,
    /// 旧键 `RaidScuttle.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `RaidScuttle.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `RaidScuttle.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `RaidScuttle.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `RaidScuttle.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `RaidScuttle.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleEmotion {
    /// 旧键 `RaidScuttle.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `RaidScuttle.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `RaidScuttle.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleFleet {
    /// 旧键 `RaidScuttle.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `RaidScuttle.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `RaidScuttle.Raid` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleRaid {
    /// 旧键 `RaidScuttle.Raid.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `RaidScuttle.Raid.UseTicket`
    #[serde(rename = "UseTicket", default)]
    pub use_ticket: Option<bool>,
}

/// `RaidScuttle.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleScheduler {
    /// 旧键 `RaidScuttle.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `RaidScuttle.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `RaidScuttle.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `RaidScuttle.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `RaidScuttle.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleStopCondition {
    /// 旧键 `RaidScuttle.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `RaidScuttle.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `RaidScuttle.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `RaidScuttle.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `RaidScuttle.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `RaidScuttle.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `RaidScuttle.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `RaidScuttle.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttleSubmarine {
    /// 旧键 `RaidScuttle.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `RaidScuttle.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `RaidScuttle.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `RaidScuttle.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `RaidScuttle` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RaidScuttle {
    /// 旧键 `RaidScuttle.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<RaidScuttleCampaign>,
    /// 旧键 `RaidScuttle.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<RaidScuttleEmotion>,
    /// 旧键 `RaidScuttle.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<RaidScuttleFleet>,
    /// 旧键 `RaidScuttle.Raid.*`
    #[serde(rename = "Raid", default)]
    pub raid: Option<RaidScuttleRaid>,
    /// 旧键 `RaidScuttle.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<RaidScuttleScheduler>,
    /// 旧键 `RaidScuttle.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<RaidScuttleStopCondition>,
    /// 旧键 `RaidScuttle.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<RaidScuttleSubmarine>,
}

/// `Research.Research` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResearchResearch {
    /// 旧键 `Research.Research.UseCoin`
    #[serde(rename = "UseCoin", default)]
    pub use_coin: Option<String>,
    /// 旧键 `Research.Research.UseCube`
    #[serde(rename = "UseCube", default)]
    pub use_cube: Option<String>,
    /// 旧键 `Research.Research.UsePart`
    #[serde(rename = "UsePart", default)]
    pub use_part: Option<String>,
    /// 旧键 `Research.Research.AllowDelay`
    #[serde(rename = "AllowDelay", default)]
    pub allow_delay: Option<bool>,
    /// 旧键 `Research.Research.AllowGenreT`
    #[serde(rename = "AllowGenreT", default)]
    pub allow_genre_t: Option<bool>,
    /// 旧键 `Research.Research.CustomFilter`
    #[serde(rename = "CustomFilter", default)]
    pub custom_filter: Option<String>,
    /// 旧键 `Research.Research.PresetFilter`
    #[serde(rename = "PresetFilter", default)]
    pub preset_filter: Option<String>,
    /// 旧键 `Research.Research.RemainingCommissions`
    #[serde(rename = "RemainingCommissions", default)]
    pub remaining_commissions: Option<String>,
}

/// `Research.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ResearchScheduler {
    /// 旧键 `Research.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Research.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Research.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Research.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Research` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Research {
    /// 旧键 `Research.Research.*`
    #[serde(rename = "Research", default)]
    pub research: Option<ResearchResearch>,
    /// 旧键 `Research.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ResearchScheduler>,
}

/// `Restart.Restart` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RestartRestart {
    /// 旧键 `Restart.Restart.ClearCache`
    #[serde(rename = "ClearCache", default)]
    pub clear_cache: Option<bool>,
    /// 旧键 `Restart.Restart.RandomDelay`
    #[serde(rename = "RandomDelay", default)]
    pub random_delay: Option<String>,
    /// 旧键 `Restart.Restart.LoginWaitTimeout`
    #[serde(rename = "LoginWaitTimeout", default)]
    pub login_wait_timeout: Option<String>,
}

/// `Restart.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RestartScheduler {
    /// 旧键 `Restart.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Restart.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Restart.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Restart` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Restart {
    /// 旧键 `Restart.Restart.*`
    #[serde(rename = "Restart", default)]
    pub restart: Option<RestartRestart>,
    /// 旧键 `Restart.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<RestartScheduler>,
}

/// `Reward.Reward` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RewardReward {
    /// 旧键 `Reward.Reward.CollectExp`
    #[serde(rename = "CollectExp", default)]
    pub collect_exp: Option<bool>,
    /// 旧键 `Reward.Reward.CollectOil`
    #[serde(rename = "CollectOil", default)]
    pub collect_oil: Option<bool>,
    /// 旧键 `Reward.Reward.CollectCoin`
    #[serde(rename = "CollectCoin", default)]
    pub collect_coin: Option<bool>,
    /// 旧键 `Reward.Reward.CollectMission`
    #[serde(rename = "CollectMission", default)]
    pub collect_mission: Option<bool>,
    /// 旧键 `Reward.Reward.CollectWeeklyMission`
    #[serde(rename = "CollectWeeklyMission", default)]
    pub collect_weekly_mission: Option<bool>,
}

/// `Reward.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct RewardScheduler {
    /// 旧键 `Reward.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Reward.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Reward.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Reward.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Reward` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Reward {
    /// 旧键 `Reward.Reward.*`
    #[serde(rename = "Reward", default)]
    pub reward: Option<RewardReward>,
    /// 旧键 `Reward.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<RewardScheduler>,
}

/// `Shipyard.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShipyardScheduler {
    /// 旧键 `Shipyard.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Shipyard.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Shipyard.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Shipyard.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Shipyard.Shipyard` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShipyardShipyard {
    /// 旧键 `Shipyard.Shipyard.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `Shipyard.Shipyard.BuyAmount`
    #[serde(rename = "BuyAmount", default)]
    pub buy_amount: Option<String>,
    /// 旧键 `Shipyard.Shipyard.ShipIndex`
    #[serde(rename = "ShipIndex", default)]
    pub ship_index: Option<String>,
    /// 旧键 `Shipyard.Shipyard.ResearchSeries`
    #[serde(rename = "ResearchSeries", default)]
    pub research_series: Option<String>,
}

/// `Shipyard.ShipyardDr` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShipyardShipyardDr {
    /// 旧键 `Shipyard.ShipyardDr.LastRun`
    #[serde(rename = "LastRun", default)]
    pub last_run: Option<String>,
    /// 旧键 `Shipyard.ShipyardDr.BuyAmount`
    #[serde(rename = "BuyAmount", default)]
    pub buy_amount: Option<String>,
    /// 旧键 `Shipyard.ShipyardDr.ShipIndex`
    #[serde(rename = "ShipIndex", default)]
    pub ship_index: Option<String>,
    /// 旧键 `Shipyard.ShipyardDr.ResearchSeries`
    #[serde(rename = "ResearchSeries", default)]
    pub research_series: Option<String>,
}

/// `Shipyard` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Shipyard {
    /// 旧键 `Shipyard.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ShipyardScheduler>,
    /// 旧键 `Shipyard.Shipyard.*`
    #[serde(rename = "Shipyard", default)]
    pub shipyard: Option<ShipyardShipyard>,
    /// 旧键 `Shipyard.ShipyardDr.*`
    #[serde(rename = "ShipyardDr", default)]
    pub shipyard_dr: Option<ShipyardShipyardDr>,
}

/// `ShopFrequent.GeneralShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopFrequentGeneralShop {
    /// 旧键 `ShopFrequent.GeneralShop.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopFrequent.GeneralShop.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
    /// 旧键 `ShopFrequent.GeneralShop.Refresh`
    #[serde(rename = "Refresh", default)]
    pub refresh: Option<bool>,
    /// 旧键 `ShopFrequent.GeneralShop.UseGems`
    #[serde(rename = "UseGems", default)]
    pub use_gems: Option<bool>,
    /// 旧键 `ShopFrequent.GeneralShop.BuySkinBox`
    #[serde(rename = "BuySkinBox", default)]
    pub buy_skin_box: Option<bool>,
    /// 旧键 `ShopFrequent.GeneralShop.ConsumeCoins`
    #[serde(rename = "ConsumeCoins", default)]
    pub consume_coins: Option<String>,
    /// 旧键 `ShopFrequent.GeneralShop.OverflowCoins`
    #[serde(rename = "OverflowCoins", default)]
    pub overflow_coins: Option<String>,
}

/// `ShopFrequent.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopFrequentScheduler {
    /// 旧键 `ShopFrequent.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopFrequent.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `ShopFrequent.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `ShopFrequent.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `ShopFrequent` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopFrequent {
    /// 旧键 `ShopFrequent.GeneralShop.*`
    #[serde(rename = "GeneralShop", default)]
    pub general_shop: Option<ShopFrequentGeneralShop>,
    /// 旧键 `ShopFrequent.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ShopFrequentScheduler>,
}

/// `ShopOnce.CoreShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnceCoreShop {
    /// 旧键 `ShopOnce.CoreShop.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopOnce.CoreShop.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
}

/// `ShopOnce.GuildShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnceGuildShop {
    /// 旧键 `ShopOnce.GuildShop.PR1`
    #[serde(rename = "PR1", default)]
    pub pr1: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.PR2`
    #[serde(rename = "PR2", default)]
    pub pr2: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.PR3`
    #[serde(rename = "PR3", default)]
    pub pr3: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.BOX_T3`
    #[serde(rename = "BOX_T3", default)]
    pub box_t3: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.BOX_T4`
    #[serde(rename = "BOX_T4", default)]
    pub box_t4: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopOnce.GuildShop.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.BOOK_T2`
    #[serde(rename = "BOOK_T2", default)]
    pub book_t2: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.BOOK_T3`
    #[serde(rename = "BOOK_T3", default)]
    pub book_t3: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.Refresh`
    #[serde(rename = "Refresh", default)]
    pub refresh: Option<bool>,
    /// 旧键 `ShopOnce.GuildShop.PLATE_T2`
    #[serde(rename = "PLATE_T2", default)]
    pub plate_t2: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.PLATE_T3`
    #[serde(rename = "PLATE_T3", default)]
    pub plate_t3: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.PLATE_T4`
    #[serde(rename = "PLATE_T4", default)]
    pub plate_t4: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.RETROFIT_T2`
    #[serde(rename = "RETROFIT_T2", default)]
    pub retrofit_t2: Option<String>,
    /// 旧键 `ShopOnce.GuildShop.RETROFIT_T3`
    #[serde(rename = "RETROFIT_T3", default)]
    pub retrofit_t3: Option<String>,
}

/// `ShopOnce.MedalShop2` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnceMedalShop2 {
    /// 旧键 `ShopOnce.MedalShop2.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopOnce.MedalShop2.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.PLATE_T1`
    #[serde(rename = "PLATE_T1", default)]
    pub plate_t1: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.PLATE_T2`
    #[serde(rename = "PLATE_T2", default)]
    pub plate_t2: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.PLATE_T3`
    #[serde(rename = "PLATE_T3", default)]
    pub plate_t3: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.RETROFIT_T1`
    #[serde(rename = "RETROFIT_T1", default)]
    pub retrofit_t1: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.RETROFIT_T2`
    #[serde(rename = "RETROFIT_T2", default)]
    pub retrofit_t2: Option<String>,
    /// 旧键 `ShopOnce.MedalShop2.RETROFIT_T3`
    #[serde(rename = "RETROFIT_T3", default)]
    pub retrofit_t3: Option<String>,
}

/// `ShopOnce.MeritShop` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnceMeritShop {
    /// 旧键 `ShopOnce.MeritShop.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopOnce.MeritShop.Filter`
    #[serde(rename = "Filter", default)]
    pub filter: Option<String>,
    /// 旧键 `ShopOnce.MeritShop.Refresh`
    #[serde(rename = "Refresh", default)]
    pub refresh: Option<bool>,
}

/// `ShopOnce.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnceScheduler {
    /// 旧键 `ShopOnce.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ShopOnce.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `ShopOnce.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `ShopOnce.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `ShopOnce` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ShopOnce {
    /// 旧键 `ShopOnce.CoreShop.*`
    #[serde(rename = "CoreShop", default)]
    pub core_shop: Option<ShopOnceCoreShop>,
    /// 旧键 `ShopOnce.GuildShop.*`
    #[serde(rename = "GuildShop", default)]
    pub guild_shop: Option<ShopOnceGuildShop>,
    /// 旧键 `ShopOnce.MedalShop2.*`
    #[serde(rename = "MedalShop2", default)]
    pub medal_shop2: Option<ShopOnceMedalShop2>,
    /// 旧键 `ShopOnce.MeritShop.*`
    #[serde(rename = "MeritShop", default)]
    pub merit_shop: Option<ShopOnceMeritShop>,
    /// 旧键 `ShopOnce.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ShopOnceScheduler>,
}

/// `Tactical.AddNewStudent` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TacticalAddNewStudent {
    /// 旧键 `Tactical.AddNewStudent.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Tactical.AddNewStudent.Favorite`
    #[serde(rename = "Favorite", default)]
    pub favorite: Option<bool>,
    /// 旧键 `Tactical.AddNewStudent.MinLevel`
    #[serde(rename = "MinLevel", default)]
    pub min_level: Option<String>,
}

/// `Tactical.ControlExpOverflow` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TacticalControlExpOverflow {
    /// 旧键 `Tactical.ControlExpOverflow.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Tactical.ControlExpOverflow.T1Allow`
    #[serde(rename = "T1Allow", default)]
    pub t1_allow: Option<String>,
    /// 旧键 `Tactical.ControlExpOverflow.T2Allow`
    #[serde(rename = "T2Allow", default)]
    pub t2_allow: Option<String>,
    /// 旧键 `Tactical.ControlExpOverflow.T3Allow`
    #[serde(rename = "T3Allow", default)]
    pub t3_allow: Option<String>,
    /// 旧键 `Tactical.ControlExpOverflow.T4Allow`
    #[serde(rename = "T4Allow", default)]
    pub t4_allow: Option<String>,
}

/// `Tactical.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TacticalScheduler {
    /// 旧键 `Tactical.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `Tactical.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `Tactical.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `Tactical.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `Tactical.Tactical` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct TacticalTactical {
    /// 旧键 `Tactical.Tactical.TacticalFilter`
    #[serde(rename = "TacticalFilter", default)]
    pub tactical_filter: Option<String>,
    /// 旧键 `Tactical.Tactical.SkillAutoSwitch`
    #[serde(rename = "SkillAutoSwitch", default)]
    pub skill_auto_switch: Option<bool>,
    /// 旧键 `Tactical.Tactical.RapidTrainingSlot`
    #[serde(rename = "RapidTrainingSlot", default)]
    pub rapid_training_slot: Option<String>,
}

/// `Tactical` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Tactical {
    /// 旧键 `Tactical.AddNewStudent.*`
    #[serde(rename = "AddNewStudent", default)]
    pub add_new_student: Option<TacticalAddNewStudent>,
    /// 旧键 `Tactical.ControlExpOverflow.*`
    #[serde(rename = "ControlExpOverflow", default)]
    pub control_exp_overflow: Option<TacticalControlExpOverflow>,
    /// 旧键 `Tactical.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<TacticalScheduler>,
    /// 旧键 `Tactical.Tactical.*`
    #[serde(rename = "Tactical", default)]
    pub tactical: Option<TacticalTactical>,
}

/// `ThreeOilLowCost.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostCampaign {
    /// 旧键 `ThreeOilLowCost.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `ThreeOilLowCost.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `ThreeOilLowCost.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `ThreeOilLowCost.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `ThreeOilLowCost.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostEmotion {
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `ThreeOilLowCost.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
}

/// `ThreeOilLowCost.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostFleet {
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `ThreeOilLowCost.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `ThreeOilLowCost.GemsFarming` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostGemsFarming {
    /// 旧键 `ThreeOilLowCost.GemsFarming.CommonCV`
    #[serde(rename = "CommonCV", default)]
    pub common_cv: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.CommonDD`
    #[serde(rename = "CommonDD", default)]
    pub common_dd: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.EquipmentCode`
    #[serde(rename = "EquipmentCode", default)]
    pub equipment_code: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.ChangeFlagship`
    #[serde(rename = "ChangeFlagship", default)]
    pub change_flagship: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.ChangeVanguard`
    #[serde(rename = "ChangeVanguard", default)]
    pub change_vanguard: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.CommonCVFilter`
    #[serde(rename = "CommonCVFilter", default)]
    pub common_cvfilter: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.CommonDDFilter`
    #[serde(rename = "CommonDDFilter", default)]
    pub common_ddfilter: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.CommissionLimit`
    #[serde(rename = "CommissionLimit", default)]
    pub commission_limit: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.UseEmotionFirst`
    #[serde(rename = "UseEmotionFirst", default)]
    pub use_emotion_first: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.VanguardLevelMax`
    #[serde(rename = "VanguardLevelMax", default)]
    pub vanguard_level_max: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.VanguardLevelMin`
    #[serde(rename = "VanguardLevelMin", default)]
    pub vanguard_level_min: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.IgnoreEmotionWarning`
    #[serde(rename = "IgnoreEmotionWarning", default)]
    pub ignore_emotion_warning: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.AllowLowVanguardLevel`
    #[serde(rename = "AllowLowVanguardLevel", default)]
    pub allow_low_vanguard_level: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.DelayTaskIFNoFlagship`
    #[serde(rename = "DelayTaskIFNoFlagship", default)]
    pub delay_task_ifno_flagship: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.AllowHighFlagshipLevel`
    #[serde(rename = "AllowHighFlagshipLevel", default)]
    pub allow_high_flagship_level: Option<bool>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.HighValueCommissionReserve`
    #[serde(rename = "HighValueCommissionReserve", default)]
    pub high_value_commission_reserve: Option<String>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.HighValueCommissionFilterCount`
    #[serde(rename = "HighValueCommissionFilterCount", default)]
    pub high_value_commission_filter_count: Option<String>,
}

/// `ThreeOilLowCost.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostScheduler {
    /// 旧键 `ThreeOilLowCost.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `ThreeOilLowCost.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `ThreeOilLowCost.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `ThreeOilLowCost.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `ThreeOilLowCost.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostStopCondition {
    /// 旧键 `ThreeOilLowCost.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `ThreeOilLowCost.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
}

/// `ThreeOilLowCost.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCostSubmarine {
    /// 旧键 `ThreeOilLowCost.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `ThreeOilLowCost.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
}

/// `ThreeOilLowCost` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct ThreeOilLowCost {
    /// 旧键 `ThreeOilLowCost.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<ThreeOilLowCostCampaign>,
    /// 旧键 `ThreeOilLowCost.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<ThreeOilLowCostEmotion>,
    /// 旧键 `ThreeOilLowCost.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<ThreeOilLowCostFleet>,
    /// 旧键 `ThreeOilLowCost.GemsFarming.*`
    #[serde(rename = "GemsFarming", default)]
    pub gems_farming: Option<ThreeOilLowCostGemsFarming>,
    /// 旧键 `ThreeOilLowCost.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<ThreeOilLowCostScheduler>,
    /// 旧键 `ThreeOilLowCost.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<ThreeOilLowCostStopCondition>,
    /// 旧键 `ThreeOilLowCost.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<ThreeOilLowCostSubmarine>,
}

/// `WarArchives.Campaign` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesCampaign {
    /// 旧键 `WarArchives.Campaign.Name`
    #[serde(rename = "Name", default)]
    pub name: Option<String>,
    /// 旧键 `WarArchives.Campaign.Event`
    #[serde(rename = "Event", default)]
    pub event: Option<String>,
    /// 旧键 `WarArchives.Campaign.Use2xBook`
    #[serde(rename = "Use2xBook", default)]
    pub use2x_book: Option<bool>,
    /// 旧键 `WarArchives.Campaign.UseClearMode`
    #[serde(rename = "UseClearMode", default)]
    pub use_clear_mode: Option<bool>,
    /// 旧键 `WarArchives.Campaign.UseFleetLock`
    #[serde(rename = "UseFleetLock", default)]
    pub use_fleet_lock: Option<bool>,
    /// 旧键 `WarArchives.Campaign.UseAutoSearch`
    #[serde(rename = "UseAutoSearch", default)]
    pub use_auto_search: Option<bool>,
    /// 旧键 `WarArchives.Campaign.DefeatWithdraw`
    #[serde(rename = "DefeatWithdraw", default)]
    pub defeat_withdraw: Option<String>,
    /// 旧键 `WarArchives.Campaign.UseRecommendFleet`
    #[serde(rename = "UseRecommendFleet", default)]
    pub use_recommend_fleet: Option<bool>,
}

/// `WarArchives.Emotion` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesEmotion {
    /// 旧键 `WarArchives.Emotion.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet1Oath`
    #[serde(rename = "Fleet1Oath", default)]
    pub fleet1_oath: Option<bool>,
    /// 旧键 `WarArchives.Emotion.Fleet2Oath`
    #[serde(rename = "Fleet2Oath", default)]
    pub fleet2_oath: Option<bool>,
    /// 旧键 `WarArchives.Emotion.Fleet1Onsen`
    #[serde(rename = "Fleet1Onsen", default)]
    pub fleet1_onsen: Option<bool>,
    /// 旧键 `WarArchives.Emotion.Fleet1Value`
    #[serde(rename = "Fleet1Value", default)]
    pub fleet1_value: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet2Onsen`
    #[serde(rename = "Fleet2Onsen", default)]
    pub fleet2_onsen: Option<bool>,
    /// 旧键 `WarArchives.Emotion.Fleet2Value`
    #[serde(rename = "Fleet2Value", default)]
    pub fleet2_value: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet1Record`
    #[serde(rename = "Fleet1Record", default)]
    pub fleet1_record: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet2Record`
    #[serde(rename = "Fleet2Record", default)]
    pub fleet2_record: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet1Control`
    #[serde(rename = "Fleet1Control", default)]
    pub fleet1_control: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet1Recover`
    #[serde(rename = "Fleet1Recover", default)]
    pub fleet1_recover: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet2Control`
    #[serde(rename = "Fleet2Control", default)]
    pub fleet2_control: Option<String>,
    /// 旧键 `WarArchives.Emotion.Fleet2Recover`
    #[serde(rename = "Fleet2Recover", default)]
    pub fleet2_recover: Option<String>,
}

/// `WarArchives.EnemyPriority` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesEnemyPriority {
    /// 旧键 `WarArchives.EnemyPriority.EnemyScaleBalanceWeight`
    #[serde(rename = "EnemyScaleBalanceWeight", default)]
    pub enemy_scale_balance_weight: Option<String>,
}

/// `WarArchives.Fleet` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesFleet {
    /// 旧键 `WarArchives.Fleet.Fleet1`
    #[serde(rename = "Fleet1", default)]
    pub fleet1: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet2`
    #[serde(rename = "Fleet2", default)]
    pub fleet2: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet1Mode`
    #[serde(rename = "Fleet1Mode", default)]
    pub fleet1_mode: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet1Step`
    #[serde(rename = "Fleet1Step", default)]
    pub fleet1_step: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet2Mode`
    #[serde(rename = "Fleet2Mode", default)]
    pub fleet2_mode: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet2Step`
    #[serde(rename = "Fleet2Step", default)]
    pub fleet2_step: Option<String>,
    /// 旧键 `WarArchives.Fleet.FleetOrder`
    #[serde(rename = "FleetOrder", default)]
    pub fleet_order: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet1Formation`
    #[serde(rename = "Fleet1Formation", default)]
    pub fleet1_formation: Option<String>,
    /// 旧键 `WarArchives.Fleet.Fleet2Formation`
    #[serde(rename = "Fleet2Formation", default)]
    pub fleet2_formation: Option<String>,
    /// 旧键 `WarArchives.Fleet.SkipPreparation`
    #[serde(rename = "SkipPreparation", default)]
    pub skip_preparation: Option<bool>,
}

/// `WarArchives.HpControl` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesHpControl {
    /// 旧键 `WarArchives.HpControl.UseHpBalance`
    #[serde(rename = "UseHpBalance", default)]
    pub use_hp_balance: Option<bool>,
    /// 旧键 `WarArchives.HpControl.HpBalanceWeight`
    #[serde(rename = "HpBalanceWeight", default)]
    pub hp_balance_weight: Option<String>,
    /// 旧键 `WarArchives.HpControl.UseLowHpRetreat`
    #[serde(rename = "UseLowHpRetreat", default)]
    pub use_low_hp_retreat: Option<bool>,
    /// 旧键 `WarArchives.HpControl.UseEmergencyRepair`
    #[serde(rename = "UseEmergencyRepair", default)]
    pub use_emergency_repair: Option<bool>,
    /// 旧键 `WarArchives.HpControl.HpBalanceThreshold`
    #[serde(rename = "HpBalanceThreshold", default)]
    pub hp_balance_threshold: Option<String>,
    /// 旧键 `WarArchives.HpControl.LowHpRetreatThreshold`
    #[serde(rename = "LowHpRetreatThreshold", default)]
    pub low_hp_retreat_threshold: Option<String>,
    /// 旧键 `WarArchives.HpControl.RepairUseMultiThreshold`
    #[serde(rename = "RepairUseMultiThreshold", default)]
    pub repair_use_multi_threshold: Option<String>,
    /// 旧键 `WarArchives.HpControl.RepairUseSingleThreshold`
    #[serde(rename = "RepairUseSingleThreshold", default)]
    pub repair_use_single_threshold: Option<String>,
}

/// `WarArchives.Scheduler` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesScheduler {
    /// 旧键 `WarArchives.Scheduler.Enable`
    #[serde(rename = "Enable", default)]
    pub enable: Option<bool>,
    /// 旧键 `WarArchives.Scheduler.NextRun`
    #[serde(rename = "NextRun", default)]
    pub next_run: Option<String>,
    /// 旧键 `WarArchives.Scheduler.Sensitive`
    #[serde(rename = "Sensitive", default)]
    pub sensitive: Option<bool>,
    /// 旧键 `WarArchives.Scheduler.PushNotification`
    #[serde(rename = "PushNotification", default)]
    pub push_notification: Option<bool>,
}

/// `WarArchives.StopCondition` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesStopCondition {
    /// 旧键 `WarArchives.StopCondition.OilLimit`
    #[serde(rename = "OilLimit", default)]
    pub oil_limit: Option<String>,
    /// 旧键 `WarArchives.StopCondition.RunCount`
    #[serde(rename = "RunCount", default)]
    pub run_count: Option<String>,
    /// 旧键 `WarArchives.StopCondition.CoinLimit`
    #[serde(rename = "CoinLimit", default)]
    pub coin_limit: Option<String>,
    /// 旧键 `WarArchives.StopCondition.GetNewShip`
    #[serde(rename = "GetNewShip", default)]
    pub get_new_ship: Option<bool>,
    /// 旧键 `WarArchives.StopCondition.ReachLevel`
    #[serde(rename = "ReachLevel", default)]
    pub reach_level: Option<String>,
    /// 旧键 `WarArchives.StopCondition.StageIncrease`
    #[serde(rename = "StageIncrease", default)]
    pub stage_increase: Option<bool>,
    /// 旧键 `WarArchives.StopCondition.MapAchievement`
    #[serde(rename = "MapAchievement", default)]
    pub map_achievement: Option<String>,
}

/// `WarArchives.Submarine` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesSubmarine {
    /// 旧键 `WarArchives.Submarine.Mode`
    #[serde(rename = "Mode", default)]
    pub mode: Option<String>,
    /// 旧键 `WarArchives.Submarine.Fleet`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<String>,
    /// 旧键 `WarArchives.Submarine.AutoSearchMode`
    #[serde(rename = "AutoSearchMode", default)]
    pub auto_search_mode: Option<String>,
    /// 旧键 `WarArchives.Submarine.DistanceToBoss`
    #[serde(rename = "DistanceToBoss", default)]
    pub distance_to_boss: Option<String>,
}

/// `WarArchives.WarArchives` 组的配置。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchivesWarArchives {
    /// 旧键 `WarArchives.WarArchives.DailyRunCount`
    #[serde(rename = "DailyRunCount", default)]
    pub daily_run_count: Option<String>,
}

/// `WarArchives` 任务的配置（三层嵌套，与旧配置同形）。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct WarArchives {
    /// 旧键 `WarArchives.Campaign.*`
    #[serde(rename = "Campaign", default)]
    pub campaign: Option<WarArchivesCampaign>,
    /// 旧键 `WarArchives.Emotion.*`
    #[serde(rename = "Emotion", default)]
    pub emotion: Option<WarArchivesEmotion>,
    /// 旧键 `WarArchives.EnemyPriority.*`
    #[serde(rename = "EnemyPriority", default)]
    pub enemy_priority: Option<WarArchivesEnemyPriority>,
    /// 旧键 `WarArchives.Fleet.*`
    #[serde(rename = "Fleet", default)]
    pub fleet: Option<WarArchivesFleet>,
    /// 旧键 `WarArchives.HpControl.*`
    #[serde(rename = "HpControl", default)]
    pub hp_control: Option<WarArchivesHpControl>,
    /// 旧键 `WarArchives.Scheduler.*`
    #[serde(rename = "Scheduler", default)]
    pub scheduler: Option<WarArchivesScheduler>,
    /// 旧键 `WarArchives.StopCondition.*`
    #[serde(rename = "StopCondition", default)]
    pub stop_condition: Option<WarArchivesStopCondition>,
    /// 旧键 `WarArchives.Submarine.*`
    #[serde(rename = "Submarine", default)]
    pub submarine: Option<WarArchivesSubmarine>,
    /// 旧键 `WarArchives.WarArchives.*`
    #[serde(rename = "WarArchives", default)]
    pub war_archives: Option<WarArchivesWarArchives>,
}

/// 全部任务的配置根。
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Tasks {
    /// `Alas` 任务
    #[serde(rename = "Alas", default)]
    pub alas: Option<Alas>,
    /// `Ambush11` 任务
    #[serde(rename = "Ambush11", default)]
    pub ambush11: Option<Ambush11>,
    /// `AutoEquip` 任务
    #[serde(rename = "AutoEquip", default)]
    pub auto_equip: Option<AutoEquip>,
    /// `Awaken` 任务
    #[serde(rename = "Awaken", default)]
    pub awaken: Option<Awaken>,
    /// `AzurLaneUncensored` 任务
    #[serde(rename = "AzurLaneUncensored", default)]
    pub azur_lane_uncensored: Option<AzurLaneUncensored>,
    /// `Benchmark` 任务
    #[serde(rename = "Benchmark", default)]
    pub benchmark: Option<Benchmark>,
    /// `BoxDisassemble` 任务
    #[serde(rename = "BoxDisassemble", default)]
    pub box_disassemble: Option<BoxDisassemble>,
    /// `Coalition` 任务
    #[serde(rename = "Coalition", default)]
    pub coalition: Option<Coalition>,
    /// `CoalitionScuttle` 任务
    #[serde(rename = "CoalitionScuttle", default)]
    pub coalition_scuttle: Option<CoalitionScuttle>,
    /// `CoalitionSp` 任务
    #[serde(rename = "CoalitionSp", default)]
    pub coalition_sp: Option<CoalitionSp>,
    /// `Commission` 任务
    #[serde(rename = "Commission", default)]
    pub commission: Option<Commission>,
    /// `Daemon` 任务
    #[serde(rename = "Daemon", default)]
    pub daemon: Option<Daemon>,
    /// `Daily` 任务
    #[serde(rename = "Daily", default)]
    pub daily: Option<Daily>,
    /// `Dashboard` 任务
    #[serde(rename = "Dashboard", default)]
    pub dashboard: Option<Dashboard>,
    /// `Dorm` 任务
    #[serde(rename = "Dorm", default)]
    pub dorm: Option<Dorm>,
    /// `EmulatorManager` 任务
    #[serde(rename = "EmulatorManager", default)]
    pub emulator_manager: Option<EmulatorManager>,
    /// `Event` 任务
    #[serde(rename = "Event", default)]
    pub event: Option<Event>,
    /// `Event2` 任务
    #[serde(rename = "Event2", default)]
    pub event2: Option<Event2>,
    /// `Event3` 任务
    #[serde(rename = "Event3", default)]
    pub event3: Option<Event3>,
    /// `EventA` 任务
    #[serde(rename = "EventA", default)]
    pub event_a: Option<EventA>,
    /// `EventB` 任务
    #[serde(rename = "EventB", default)]
    pub event_b: Option<EventB>,
    /// `EventC` 任务
    #[serde(rename = "EventC", default)]
    pub event_c: Option<EventC>,
    /// `EventD` 任务
    #[serde(rename = "EventD", default)]
    pub event_d: Option<EventD>,
    /// `EventGeneral` 任务
    #[serde(rename = "EventGeneral", default)]
    pub event_general: Option<EventGeneral>,
    /// `EventShop` 任务
    #[serde(rename = "EventShop", default)]
    pub event_shop: Option<EventShop>,
    /// `EventSp` 任务
    #[serde(rename = "EventSp", default)]
    pub event_sp: Option<EventSp>,
    /// `Exercise` 任务
    #[serde(rename = "Exercise", default)]
    pub exercise: Option<Exercise>,
    /// `Freebies` 任务
    #[serde(rename = "Freebies", default)]
    pub freebies: Option<Freebies>,
    /// `Gacha` 任务
    #[serde(rename = "Gacha", default)]
    pub gacha: Option<Gacha>,
    /// `GameManager` 任务
    #[serde(rename = "GameManager", default)]
    pub game_manager: Option<GameManager>,
    /// `GemsFarming` 任务
    #[serde(rename = "GemsFarming", default)]
    pub gems_farming: Option<GemsFarming>,
    /// `General` 任务
    #[serde(rename = "General", default)]
    pub general: Option<General>,
    /// `Guild` 任务
    #[serde(rename = "Guild", default)]
    pub guild: Option<Guild>,
    /// `Hard` 任务
    #[serde(rename = "Hard", default)]
    pub hard: Option<Hard>,
    /// `Hospital` 任务
    #[serde(rename = "Hospital", default)]
    pub hospital: Option<Hospital>,
    /// `IslandAirDrop` 任务
    #[serde(rename = "IslandAirDrop", default)]
    pub island_air_drop: Option<IslandAirDrop>,
    /// `IslandBusiness` 任务
    #[serde(rename = "IslandBusiness", default)]
    pub island_business: Option<IslandBusiness>,
    /// `IslandCargoPreparation` 任务
    #[serde(rename = "IslandCargoPreparation", default)]
    pub island_cargo_preparation: Option<IslandCargoPreparation>,
    /// `IslandDailyGather` 任务
    #[serde(rename = "IslandDailyGather", default)]
    pub island_daily_gather: Option<IslandDailyGather>,
    /// `IslandDailyInteract` 任务
    #[serde(rename = "IslandDailyInteract", default)]
    pub island_daily_interact: Option<IslandDailyInteract>,
    /// `IslandDailyOrder` 任务
    #[serde(rename = "IslandDailyOrder", default)]
    pub island_daily_order: Option<IslandDailyOrder>,
    /// `IslandFarm` 任务
    #[serde(rename = "IslandFarm", default)]
    pub island_farm: Option<IslandFarm>,
    /// `IslandGrill` 任务
    #[serde(rename = "IslandGrill", default)]
    pub island_grill: Option<IslandGrill>,
    /// `IslandJuuCoffee` 任务
    #[serde(rename = "IslandJuuCoffee", default)]
    pub island_juu_coffee: Option<IslandJuuCoffee>,
    /// `IslandJuuEatery` 任务
    #[serde(rename = "IslandJuuEatery", default)]
    pub island_juu_eatery: Option<IslandJuuEatery>,
    /// `IslandManufacture` 任务
    #[serde(rename = "IslandManufacture", default)]
    pub island_manufacture: Option<IslandManufacture>,
    /// `IslandMineForest` 任务
    #[serde(rename = "IslandMineForest", default)]
    pub island_mine_forest: Option<IslandMineForest>,
    /// `IslandPearlSell` 任务
    #[serde(rename = "IslandPearlSell", default)]
    pub island_pearl_sell: Option<IslandPearlSell>,
    /// `IslandPlan` 任务
    #[serde(rename = "IslandPlan", default)]
    pub island_plan: Option<IslandPlan>,
    /// `IslandRancher` 任务
    #[serde(rename = "IslandRancher", default)]
    pub island_rancher: Option<IslandRancher>,
    /// `IslandRestaurant` 任务
    #[serde(rename = "IslandRestaurant", default)]
    pub island_restaurant: Option<IslandRestaurant>,
    /// `IslandTeahouse` 任务
    #[serde(rename = "IslandTeahouse", default)]
    pub island_teahouse: Option<IslandTeahouse>,
    /// `Main` 任务
    #[serde(rename = "Main", default)]
    pub main: Option<Main>,
    /// `Main2` 任务
    #[serde(rename = "Main2", default)]
    pub main2: Option<Main2>,
    /// `Main3` 任务
    #[serde(rename = "Main3", default)]
    pub main3: Option<Main3>,
    /// `MaritimeEscort` 任务
    #[serde(rename = "MaritimeEscort", default)]
    pub maritime_escort: Option<MaritimeEscort>,
    /// `Meowfficer` 任务
    #[serde(rename = "Meowfficer", default)]
    pub meowfficer: Option<Meowfficer>,
    /// `Minigame` 任务
    #[serde(rename = "Minigame", default)]
    pub minigame: Option<Minigame>,
    /// `OpsiAbyssal` 任务
    #[serde(rename = "OpsiAbyssal", default)]
    pub opsi_abyssal: Option<OpsiAbyssal>,
    /// `OpsiArchive` 任务
    #[serde(rename = "OpsiArchive", default)]
    pub opsi_archive: Option<OpsiArchive>,
    /// `OpsiAshAssist` 任务
    #[serde(rename = "OpsiAshAssist", default)]
    pub opsi_ash_assist: Option<OpsiAshAssist>,
    /// `OpsiAshBeacon` 任务
    #[serde(rename = "OpsiAshBeacon", default)]
    pub opsi_ash_beacon: Option<OpsiAshBeacon>,
    /// `OpsiCrossMonth` 任务
    #[serde(rename = "OpsiCrossMonth", default)]
    pub opsi_cross_month: Option<OpsiCrossMonth>,
    /// `OpsiDaemon` 任务
    #[serde(rename = "OpsiDaemon", default)]
    pub opsi_daemon: Option<OpsiDaemon>,
    /// `OpsiDaily` 任务
    #[serde(rename = "OpsiDaily", default)]
    pub opsi_daily: Option<OpsiDaily>,
    /// `OpsiExplore` 任务
    #[serde(rename = "OpsiExplore", default)]
    pub opsi_explore: Option<OpsiExplore>,
    /// `OpsiGeneral` 任务
    #[serde(rename = "OpsiGeneral", default)]
    pub opsi_general: Option<OpsiGeneral>,
    /// `OpsiHazard1Leveling` 任务
    #[serde(rename = "OpsiHazard1Leveling", default)]
    pub opsi_hazard1_leveling: Option<OpsiHazard1Leveling>,
    /// `OpsiMeowfficerFarming` 任务
    #[serde(rename = "OpsiMeowfficerFarming", default)]
    pub opsi_meowfficer_farming: Option<OpsiMeowfficerFarming>,
    /// `OpsiMonthBoss` 任务
    #[serde(rename = "OpsiMonthBoss", default)]
    pub opsi_month_boss: Option<OpsiMonthBoss>,
    /// `OpsiObscure` 任务
    #[serde(rename = "OpsiObscure", default)]
    pub opsi_obscure: Option<OpsiObscure>,
    /// `OpsiPreventActionPointOverflow` 任务
    #[serde(rename = "OpsiPreventActionPointOverflow", default)]
    pub opsi_prevent_action_point_overflow: Option<OpsiPreventActionPointOverflow>,
    /// `OpsiScheduling` 任务
    #[serde(rename = "OpsiScheduling", default)]
    pub opsi_scheduling: Option<OpsiScheduling>,
    /// `OpsiShop` 任务
    #[serde(rename = "OpsiShop", default)]
    pub opsi_shop: Option<OpsiShop>,
    /// `OpsiSimulator` 任务
    #[serde(rename = "OpsiSimulator", default)]
    pub opsi_simulator: Option<OpsiSimulator>,
    /// `OpsiStronghold` 任务
    #[serde(rename = "OpsiStronghold", default)]
    pub opsi_stronghold: Option<OpsiStronghold>,
    /// `OpsiVoucher` 任务
    #[serde(rename = "OpsiVoucher", default)]
    pub opsi_voucher: Option<OpsiVoucher>,
    /// `PrivateQuarters` 任务
    #[serde(rename = "PrivateQuarters", default)]
    pub private_quarters: Option<PrivateQuarters>,
    /// `Raid` 任务
    #[serde(rename = "Raid", default)]
    pub raid: Option<Raid>,
    /// `RaidDaily` 任务
    #[serde(rename = "RaidDaily", default)]
    pub raid_daily: Option<RaidDaily>,
    /// `RaidScuttle` 任务
    #[serde(rename = "RaidScuttle", default)]
    pub raid_scuttle: Option<RaidScuttle>,
    /// `Research` 任务
    #[serde(rename = "Research", default)]
    pub research: Option<Research>,
    /// `Restart` 任务
    #[serde(rename = "Restart", default)]
    pub restart: Option<Restart>,
    /// `Reward` 任务
    #[serde(rename = "Reward", default)]
    pub reward: Option<Reward>,
    /// `Shipyard` 任务
    #[serde(rename = "Shipyard", default)]
    pub shipyard: Option<Shipyard>,
    /// `ShopFrequent` 任务
    #[serde(rename = "ShopFrequent", default)]
    pub shop_frequent: Option<ShopFrequent>,
    /// `ShopOnce` 任务
    #[serde(rename = "ShopOnce", default)]
    pub shop_once: Option<ShopOnce>,
    /// `Tactical` 任务
    #[serde(rename = "Tactical", default)]
    pub tactical: Option<Tactical>,
    /// `ThreeOilLowCost` 任务
    #[serde(rename = "ThreeOilLowCost", default)]
    pub three_oil_low_cost: Option<ThreeOilLowCost>,
    /// `WarArchives` 任务
    #[serde(rename = "WarArchives", default)]
    pub war_archives: Option<WarArchives>,
}
