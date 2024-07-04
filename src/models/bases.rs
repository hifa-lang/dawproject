use crate::project::xml_schema_types::{
    Arrangement, AuPlugin, Audio, BoolParameter, BoolPoint, BuiltinDevice, Channel, ClapPlugin,
    Clip, ClipSlot, Clips, Compressor, Device, DeviceRole, EnumParameter, EnumPoint, Equalizer,
    FileReference, IntegerParameter, IntegerPoint, Lane, Lanes, Limiter, Marker, Markers,
    MediaFile, Nameable, NoiseGate, Notes, Parameter, Parameters, Plugin, Point, RealParameter,
    RealPoint, Referenceable, Scene, Send, TimeSignatureParameter, TimeSignaturePoint, TimeUnit,
    Timeline, Track, Video, Vst2Plugin, Vst3Plugin, Warps,
};

macro_rules! impl_base_trait {
    ($trait_ident:ident, $base_type:ident, $fn_ident:ident, $type:ty, $($base:tt)*) => {
        impl $trait_ident for $type {
            fn $fn_ident(&self) -> Option<&$base_type> {
                Some(&self $($base)*)
            }
        }
    };
}

pub trait NameableTrait {
    fn get_nameable(&self) -> Option<&Nameable>;

    fn get_name(&self) -> Option<&str> {
        self.get_nameable()
            .map(|nameable| nameable.name.as_ref().map(|n| n.as_str()))
            .flatten()
    }

    fn get_color(&self) -> Option<&str> {
        self.get_nameable()
            .map(|nameable| nameable.color.as_ref().map(|n| n.as_str()))
            .flatten()
    }

    fn get_comment(&self) -> Option<&str> {
        self.get_nameable()
            .map(|nameable| nameable.comment.as_ref().map(|n| n.as_str()))
            .flatten()
    }
}
impl_base_trait!(NameableTrait, Nameable, get_nameable, Nameable,);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Clip, .base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Marker, .base);
//Referenceable
impl_base_trait!(NameableTrait, Nameable, get_nameable, Referenceable, .base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Scene, .base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Arrangement, .base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Send, .base.base);
// Parameter
impl_base_trait!(NameableTrait, Nameable, get_nameable, Parameter, .base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, RealParameter, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, IntegerParameter, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, BoolParameter, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, EnumParameter, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, TimeSignatureParameter, .base.base.base);
// Lane
impl_base_trait!(NameableTrait, Nameable, get_nameable, Lane, .base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Track, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Channel, .base.base.base);
// Timeline
impl_base_trait!(NameableTrait, Nameable, get_nameable, Timeline, .base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Lanes, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Notes, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Clips, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, ClipSlot, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Markers, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Warps, .base.base.base);
// MediaFile
impl_base_trait!(NameableTrait, Nameable, get_nameable, MediaFile, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Audio, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Video, .base.base.base.base);
// Device
impl_base_trait!(NameableTrait, Nameable, get_nameable, Device, .base.base);
// BuiltinDevice
impl_base_trait!(NameableTrait, Nameable, get_nameable, BuiltinDevice, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Equalizer, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Compressor, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, NoiseGate, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Limiter, .base.base.base.base);
// Plugin
impl_base_trait!(NameableTrait, Nameable, get_nameable, Plugin, .base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Vst2Plugin, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, Vst3Plugin, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, ClapPlugin, .base.base.base.base);
impl_base_trait!(NameableTrait, Nameable, get_nameable, AuPlugin, .base.base.base.base);

pub trait ReferenceableTrait: NameableTrait {
    fn get_referenceable(&self) -> Option<&Referenceable>;

    fn get_id(&self) -> Option<&str> {
        self.get_referenceable()
            .map(|referenceable| referenceable.id.as_ref().map(|n| n.as_str()))
            .flatten()
    }
}

impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Referenceable,
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Scene,
    .base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Arrangement,
    .base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Send,
    .base
);
// Parameter
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Parameter,
    .base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    RealParameter,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    IntegerParameter,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    BoolParameter,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    EnumParameter,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    TimeSignatureParameter,
    .base.base
);
// Lane
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Lane,
    .base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Track,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Channel,
    .base.base
);
// Timeline
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Timeline,
    .base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Lanes,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Notes,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Clips,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    ClipSlot,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Markers,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Warps,
    .base.base
);
// MediaFile
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    MediaFile,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Audio,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Video,
    .base.base.base
);
// Device
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Device,
    .base
);
// BuiltinDevice
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    BuiltinDevice,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Equalizer,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Compressor,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    NoiseGate,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Limiter,
    .base.base.base
);
// Plugin
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Plugin,
    .base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Vst2Plugin,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    Vst3Plugin,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    ClapPlugin,
    .base.base.base
);
impl_base_trait!(
    ReferenceableTrait,
    Referenceable,
    get_referenceable,
    AuPlugin,
    .base.base.base
);

pub trait ParameterTrait: ReferenceableTrait {
    fn get_parameter(&self) -> Option<&Parameter>;

    fn get_parameter_id(&self) -> Option<i32> {
        self.get_parameter()
            .map(|parameter| parameter.parameter_id)
            .flatten()
    }
}

impl_base_trait!(ParameterTrait, Parameter, get_parameter, Parameter,);
impl_base_trait!(ParameterTrait, Parameter, get_parameter, RealParameter, .base);
impl_base_trait!(ParameterTrait, Parameter, get_parameter, IntegerParameter, .base);
impl_base_trait!(ParameterTrait, Parameter, get_parameter, BoolParameter, .base);
impl_base_trait!(ParameterTrait, Parameter, get_parameter, EnumParameter, .base);
impl_base_trait!(ParameterTrait, Parameter, get_parameter, TimeSignatureParameter, .base);

pub trait LaneTrait: ReferenceableTrait {
    fn get_lane(&self) -> Option<&Lane>;
}

impl_base_trait!(LaneTrait, Lane, get_lane, Lane,);
impl_base_trait!(LaneTrait, Lane, get_lane, Track, .base);
impl_base_trait!(LaneTrait, Lane, get_lane, Channel, .base);

pub trait TimelineTrait: ReferenceableTrait {
    fn get_timeline(&self) -> Option<&Timeline>;

    fn get_time_unit(&self) -> Option<&TimeUnit> {
        self.get_timeline()
            .map(|timeline| timeline.time_unit.as_ref())
            .flatten()
    }
}

impl_base_trait!(TimelineTrait, Timeline, get_timeline, Timeline,);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Lanes, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Notes, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Clips, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, ClipSlot, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Markers, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Warps, .base);
// MediaFile
impl_base_trait!(TimelineTrait, Timeline, get_timeline, MediaFile, .base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Audio, .base.base);
impl_base_trait!(TimelineTrait, Timeline, get_timeline, Video, .base.base);

pub trait MediaFileTrait: TimelineTrait {
    fn get_media_file(&self) -> Option<&MediaFile>;

    fn get_file(&self) -> Option<&FileReference> {
        self.get_media_file().map(|media_file| &media_file.file)
    }
    fn get_duration(&self) -> Option<f64> {
        self.get_media_file().map(|media_file| media_file.duration)
    }
}

impl_base_trait!(MediaFileTrait, MediaFile, get_media_file, MediaFile,);
impl_base_trait!(MediaFileTrait, MediaFile, get_media_file, Audio, .base);
impl_base_trait!(MediaFileTrait, MediaFile, get_media_file, Video, .base);

pub trait DeviceTrait: ReferenceableTrait {
    fn get_device(&self) -> Option<&Device>;

    fn get_parameters(&self) -> Option<&Parameters> {
        self.get_device()
            .map(|device| device.parameters.as_ref())
            .flatten()
    }

    fn get_enabled(&self) -> Option<&BoolParameter> {
        self.get_device()
            .map(|device| device.enabled.as_ref())
            .flatten()
    }

    fn get_state(&self) -> Option<&FileReference> {
        self.get_device()
            .map(|device| device.state.as_ref())
            .flatten()
    }

    fn get_device_id(&self) -> Option<&str> {
        self.get_device()
            .map(|device| device.device_id.as_ref().map(|n| n.as_str()))
            .flatten()
    }

    fn get_device_name(&self) -> Option<&str> {
        self.get_device().map(|device| device.device_name.as_str())
    }

    fn get_device_role(&self) -> Option<&DeviceRole> {
        self.get_device().map(|device| &device.device_role)
    }

    fn get_device_vendor(&self) -> Option<&str> {
        self.get_device()
            .map(|device| device.device_vendor.as_ref().map(|n| n.as_str()))
            .flatten()
    }

    fn get_loaded(&self) -> Option<bool> {
        self.get_device().map(|device| device.loaded).flatten()
    }
}

impl_base_trait!(DeviceTrait, Device, get_device, Device,);
// BuiltinDevice
impl_base_trait!(DeviceTrait, Device, get_device, BuiltinDevice, .base);
impl_base_trait!(DeviceTrait, Device, get_device, Equalizer, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, Compressor, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, NoiseGate, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, Limiter, .base.base);
// Plugin
impl_base_trait!(DeviceTrait, Device, get_device, Plugin, .base);
impl_base_trait!(DeviceTrait, Device, get_device, Vst2Plugin, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, Vst3Plugin, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, ClapPlugin, .base.base);
impl_base_trait!(DeviceTrait, Device, get_device, AuPlugin, .base.base);

pub trait PluginTrait: DeviceTrait {
    fn get_plugin(&self) -> Option<&Plugin>;

    fn get_plugin_version(&self) -> Option<&str> {
        self.get_plugin()
            .map(|plugin| plugin.plugin_version.as_ref().map(|n| n.as_str()))
            .flatten()
    }
}

impl_base_trait!(PluginTrait, Plugin, get_plugin, Plugin,);
impl_base_trait!(PluginTrait, Plugin, get_plugin, Vst2Plugin, .base);
impl_base_trait!(PluginTrait, Plugin, get_plugin, Vst3Plugin, .base);
impl_base_trait!(PluginTrait, Plugin, get_plugin, ClapPlugin, .base);
impl_base_trait!(PluginTrait, Plugin, get_plugin, AuPlugin, .base);

pub trait BuiltinDeviceTrait: DeviceTrait {
    fn get_builtin_device(&self) -> Option<&BuiltinDevice>;
}

impl_base_trait!(
    BuiltinDeviceTrait,
    BuiltinDevice,
    get_builtin_device,
    BuiltinDevice,
);
impl_base_trait!(
    BuiltinDeviceTrait,
    BuiltinDevice,
    get_builtin_device,
    Equalizer,
    .base
);
impl_base_trait!(
    BuiltinDeviceTrait,
    BuiltinDevice,
    get_builtin_device,
    Compressor,
    .base
);
impl_base_trait!(
    BuiltinDeviceTrait,
    BuiltinDevice,
    get_builtin_device,
    NoiseGate,
    .base
);
impl_base_trait!(
    BuiltinDeviceTrait,
    BuiltinDevice,
    get_builtin_device,
    Limiter,
    .base
);

pub trait PointTrait {
    fn get_point(&self) -> Option<&Point>;

    fn get_time(&self) -> Option<&str> {
        self.get_point().map(|point| point.time.as_str())
    }
}

impl_base_trait!(PointTrait, Point, get_point, Point,);
impl_base_trait!(PointTrait, Point, get_point, BoolPoint, .base);
impl_base_trait!(PointTrait, Point, get_point, EnumPoint, .base);
impl_base_trait!(PointTrait, Point, get_point, RealPoint, .base);
impl_base_trait!(PointTrait, Point, get_point, IntegerPoint, .base);
impl_base_trait!(PointTrait, Point, get_point, TimeSignaturePoint, .base);
