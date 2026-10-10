use crate::rkmpi::rk_const::*;
use libc::c_void;
use std::os::raw::c_char;
use crate::rkmpi::rk_codec::*;

pub type RkBool = bool;
pub type RkU32 = u32;
pub type RkS32 = i32;

//==================================RK MPI COMMON==========================================

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PointS
{
    x: RkS32 ,
    y: RkS32
} 


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RkModIdE {
    RkIdCmpi    = 0,
    RkIdMb      = 1,
    RkIdSys     = 2,
    RkIdRgn     = 3,
    RkIdVenc    = 4,
    RkIdVdec    = 5,
    RkIdVpss    = 6,
    RkIdVgs     = 7,
    RkIdVi      = 8,
    RkIdVo      = 9,
    RkIdAi      = 10,
    RkIdAo      = 11,
    RkIdAenc    = 12,
    RkIdAdec    = 13,
    RkIdTde     = 14,
    RkIdIsp     = 15,
    RkIdWbc     = 16,
    RkIdAvs     = 17,
    RkIdRga     = 18,
    RkIdAf      = 19,
    RkIdIvs     = 20,
    RkIdGpu     = 21,
    RkIdNn      = 22,
    RkIdAiisp   = 23,
    RkIdButt,
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoFrame {
    pub p_mb_blk: *mut libc::c_void,
    pub width: u32,
    pub height: u32,
    pub vir_width: u32,
    pub vir_height: u32,
    pub field: u32,
    pub pixel_format: u32,
    pub video_format: u32,
    pub compress_mode: u32,
    pub dynamic_range: u32,
    pub color_gamut: u32,
    pub p_vir_addr: [*mut libc::c_void; 2],
    pub time_ref: u32,
    pub pts: u64,
    pub private_data: u64,
    pub frame_flag: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbPool {
    pub pool_id: u32,
    pub padding: u32,  
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoFrameInfo {
    pub vi_frame: VideoFrame,   
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbPoolConfig {
    pub mb_size: u64,
    pub mb_cnt: u32,
    pub remap_mode: u32,
    pub alloc_type: u32,
    pub dma_type: u32,
    pub pre_alloc: u32,
    pub not_delete: u32,
}

#[repr(C)]
pub struct PicBufAttr {
    pub width: u32,
    pub height: u32,
    pub comp_mode: u32,
    pub pixel_format: u32,
}

#[repr(C)]
pub struct MbPicCalResult {
    pub mb_size: u32,
    pub vir_width: u32,
    pub vir_height: u32,
}

#[repr(C)]
pub struct MbExtConfig {
    pub vir_addr: *mut u8,       
    pub phy_addr: u64,           
    pub fd: i32,                 
    pub size: u64,               
    pub free_cb: *mut c_void,    
    pub opaque: *mut c_void,     
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RectS {
    pub s32_x: i32,
    pub s32_y: i32,
    pub u32_width: u32,
    pub u32_height: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SizeS {
    pub u32_width: u32,
    pub u32_height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MppChnS {
    pub en_mod_id: RkModIdE, 
    pub s32_dev_id: i32,
    pub s32_chn_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FrameRateCtrlS {
    pub s32_src_frm_rate: RkS32,
    pub s32_dst_frm_rate: RkS32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBitwidthE {
    DataBitwidth8 = 0,
    DataBitwidth10,
    DataBitwidth12,
    DataBitwidth14,
    DataBitwidth16,
    DataBitwidthButt,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressModeE {
    CompressModeNone = 0,
    CompressModeNoneDgc,
    CompressModeNoneFbc,
    CompressModeNoneButt,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormatE {
    RkFmtYuv420Sp         = 0x00000000,
    RkFmtYuv420Sp10Bit    = 0x00000001,
    RkFmtYuv422Sp         = 0x00000002,
    RkFmtYuv422Sp10Bit    = 0x00000003,
    RkFmtYuv420P          = 0x00000004,
    RkFmtYuv420PVu        = 0x00000005,
    RkFmtYuv420SpVu       = 0x00000006,
    RkFmtYuv422P          = 0x00000007,
    RkFmtYuv422SpVu       = 0x00000008,
    RkFmtYuv422Yuyv       = 0x00000009,
    RkFmtYuv422Uyvy       = 0x0000000a,
    RkFmtYuv400Sp         = 0x0000000b,
    RkFmtYuv440Sp         = 0x0000000c,
    RkFmtYuv411Sp         = 0x0000000d,
    RkFmtYuv444           = 0x0000000e,
    RkFmtYuv444Sp         = 0x0000000f,
    RkFmtYuv444P          = 0x00000010,
    RkFmtYuv422Yvyu       = 0x00000011,
    RkFmtYuv422Vyuy       = 0x00000012,
    RkFmtYuvButt          = 0x00000013,

    RkFmtRgb565           = 0x00010000,
    RkFmtBgr565           = 0x00010001,
    RkFmtRgb555           = 0x00010002,
    RkFmtBgr555           = 0x00010003,
    RkFmtRgb444           = 0x00010004,
    RkFmtBgr444           = 0x00010005,
    RkFmtRgb888           = 0x00010006,
    RkFmtBgr888           = 0x00010007,
    RkFmtRgb101010        = 0x00010008,
    RkFmtBgr101010        = 0x00010009,
    RkFmtArgb1555         = 0x0001000a,
    RkFmtAbgr1555         = 0x0001000b,
    RkFmtArgb4444         = 0x0001000c,
    RkFmtAbgr4444         = 0x0001000d,
    RkFmtArgb8565         = 0x0001000e,
    RkFmtAbgr8565         = 0x0001000f,
    RkFmtArgb8888         = 0x00010010,
    RkFmtAbgr8888         = 0x00010011,
    RkFmtBgra8888         = 0x00010012,
    RkFmtRgba8888         = 0x00010013,
    RkFmtRgba5551         = 0x00010014,
    RkFmtBgra5551         = 0x00010015,
    RkFmtBgra4444         = 0x00010016,
    RkFmtRgba4444         = 0x00010017,
    RkFmtXbgr8888         = 0x00010018,
    RkFmtRgbButt          = 0x00010019,

    RkFmt2Bpp             = 0x0001001a,

    RkFmtRgbBayerSbggr8Bpp  = 0x00020000,
    RkFmtRgbBayerSgbrg8Bpp  = 0x00020001,
    RkFmtRgbBayerSgrbg8Bpp  = 0x00020002,
    RkFmtRgbBayerSrggb8Bpp  = 0x00020003,
    RkFmtRgbBayerSbggr10Bpp = 0x00020004,
    RkFmtRgbBayerSgbrg10Bpp = 0x00020005,
    RkFmtRgbBayerSgrbg10Bpp = 0x00020006,
    RkFmtRgbBayerSrggb10Bpp = 0x00020007,
    RkFmtRgbBayerSbggr12Bpp = 0x00020008,
    RkFmtRgbBayerSgbrg12Bpp = 0x00020009,
    RkFmtRgbBayerSgrbg12Bpp = 0x0002000a,
    RkFmtRgbBayerSrggb12Bpp = 0x0002000b,
    RtFmtRgbBayer14Bpp      = 0x0002000c,
    RkFmtRgbBayerSbggr16Bpp = 0x0002000d,
    RkFmtRgbBayerSgbrg16Bpp = 0x0002000e,
    RkFmtRgbBayerSgrbg16Bpp = 0x0002000f,
    RkFmtRgbBayerSrggb16Bpp = 0x00020010,
    RkFmtRgbBayerButt       = 0x00020011,
}


//==================================RK MPI VI==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViHdrModeE {
    ViHdrModeNone = 0,
    ViHdrMode2To1 = 1,
    ViHdrMode3To1 = 2,
    ViHdrMode4To1 = 3,
    ViHdrModeButt,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViRawMemTypeE {
    ViRawMemTypeSram = 0,
    ViRawMemTypeDdr,
    ViRawMemTypeButt,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViPipeAttrS {
    pub b_isp_bypass: RkBool,
    pub u32_max_w: RkU32,
    pub u32_max_h: RkU32,
    pub en_pix_fmt: PixelFormatE,
    pub en_compress_mode: CompressModeE,
    pub en_bit_width: DataBitwidthE,
    pub st_frame_rate: FrameRateCtrlS,
    pub en_mem_mode: ViRawMemTypeE,
    pub en_hdr_mode: ViHdrModeE,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViIspOptS {
    pub u32_buf_count: u32,
    pub u32_buf_size: u32,
    pub en_capture_type: u32,
    pub en_memory_type: u32, 
    pub a_entity_name: [i8; MAX_VI_ENTITY_NAME_LEN], 
    pub b_no_use_lib_v4l2: u32,
    pub st_max_size: SizeS,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViChnAttr {
    pub st_size: SizeS,
    pub en_pixel_format: PixelFormatE,    
    pub en_dynamic_range: u32,  
    pub en_video_format: u32,    
    pub en_compress_mode: u32,   
    pub b_mirror: u32,          
    pub b_flip: u32,            
    pub u32_depth: u32,
    pub st_frame_rate: FrameRateCtrlS,
    pub en_alloc_buf_type: u32,  
    pub st_isp_opt: ViIspOptS,
    pub st_share_buf_chn: MppChnS,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViDevAttr {
    pub en_intf_mode: u32,
    pub en_work_mode: u32,
    pub en_data_seq: u32,
    pub en_input_data_type: u32,
    pub st_max_size: SizeS,
    pub en_data_rate: u32,
    pub en_pixel_wormat: PixelFormatE,
    pub en_raw_memory_type: ViRawMemTypeE,
    pub en_buf_type: u32,
    pub u32_buf_count: u32,
    pub en_hdr_mode: ViHdrModeE,
    pub st_crop_rect: RectS
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViDevBindPipe {
    pub u32_num: u32,
    pub pipe_id: [u32; 4],
    pub b_data_offline : bool,
    pub b_user_start_pipe  : [bool;4]        
}

//==================================RK MPI VENC==========================================

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencJpegParams {
    pub q_factor: u32,
    pub y_qt: [u8;64],
    pub cb_qt: [u8; 64],   
    pub cr_qt: [u8; 64], 
    pub mcu_per_ecs: u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH264Cbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub stat_time: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencAttrH264 {
    pub level: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencAttrH265{

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencAttrMjpeg{

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencMPFCfg{
    pub large_thumb_nail_num: u8, //Rk_u8
    pub large_thumb_nail_size: SizeS
}

#[derive(Copy, Clone)]
#[repr(C)]
 pub struct VencAttrJpeg {
    pub support_dcf: u32 , //RkBool
    pub mpf_cfg: VencMPFCfg, 
    pub receive_mode: VencPicReceiveModeE 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VencPicReceiveModeE{
    VencPicReceiveSingle = 0,
    VencPicReceiveMulti,
    VencPicReceiveButt
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union VencAttrUnion {
    pub h264e: VencAttrH264,
    pub h265e: VencAttrH265, 
    pub mjpege: VencAttrMjpeg,
    pub jpege: VencAttrJpeg,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProresPackTypeE {
    ProresPackPic = 1,                           
    ProresPackButt
} 

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JpegePackTypeE {
    JpegePackEcs = 5,                            
    JpegePackApp = 6,                          
    JpegePackVdo = 7,                            
    JpegePackPic = 8,                            
    JpegePackDcf = 9,                          
    JpegePackDcfPic = 10,                     
    JpegePackButt = 11
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H264ENaluTypeE {
    H264eNaluBslice = 0,                        
    H264eNaluPslice = 1,                        
    H264eNaluIslice = 2,                        
    H264eNaluIdrslice = 5,                       
    H264eNaluSei    = 6,                        
    H264eNaluSps    = 7,                        
    H264eNaluPps    = 8,                         
    H264eNaluButt
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H265ENaluTypeE {
    H265eNaluBslice = 0,                         
    H265eNaluPslice = 1,                          
    H265eNaluIslice = 2,                        
    H265eNaluIdrslice = 19,                      
    H265eNaluVps    = 32,                        
    H265eNaluSps    = 33,                         
    H265eNaluPps    = 34,                         
    H265eNaluSei    = 39,                         
    H265eNaluButt
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorTypeE {
    MirrorNone = 0,
    MirrorHorizontal = 1, 
    MirrorVertical   = 2,  
    MirrorBoth       = 3,  
    MirrorButt
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencAttr {
    pub enc_type: RkCodecIdE, 
    pub max_pic_width: u32,
    pub max_pic_height: u32,
    pub pixel_format: PixelFormatE,
    pub mirror: MirrorTypeE, 
    pub buf_size: u32,
    pub profile: u32,
    pub by_frame: u32,   //RkBool или bool         
    pub pic_width: u32,
    pub pic_height: u32,
    pub vir_width: u32,
    pub vir_height: u32,
    pub stream_buf_cnt: u32,
    pub venc_union: VencAttrUnion,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct VencCropInfo{
    pub en_crop_type: VencCropTypeE,
    pub crop_rect: RectS,
    pub scale_rect: VencScaleRect, 
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct VencScaleRect{
    pub src: RectS,
    pub dst: RectS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VencCropTypeE{
    VencCropNone = 0,
    VencCropOnly,
    VencCropScale,
    VencCropButt
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct VencFrameRate{
    pub enable: u32, //RkBool
    pub src_frm_rate_num: i32,
    pub src_frm_rate_den: i32,
    pub dst_frm_rate_num: i32,
    pub dst_frm_rate_den: i32,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct VencChnParam{
    pub color_2_grey: u32, //RkBool
    pub priority: u32,
    pub max_strm_cnt: u32,
    pub poll_wake_up_frm_cnt: u32,
    pub crop_cfg: VencCropInfo, 
    pub frame_rate: VencFrameRate, 
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencGopAttr {
    pub gop_mode: u32,
    pub vir_idr_len: i32,
    pub max_ltr_count: u32,
    pub tsvc_preload: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencChnAttr {
    pub venc_attr: VencAttr,
    pub rc_attr: VencRcAttr,
    pub gop_attr: VencGopAttr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencRecvPicParam {
    pub recv_pic_num: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union VencDataTypeU {
    pub h264e_type: u32,
    pub jpege_type: u32,
    pub h265e_type: u32,
    pub prores_type:u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencPackInfo {
    pub pack_type: VencDataTypeU,
    pub pack_offset: u32,
    pub pack_length: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencPack {
    pub p_mb_blk: *mut libc::c_void,
    pub len: u32,
    pub pts: u64,
    pub frame_end: u32, //RkBool
    pub stream_end: u32, //RkBool
    pub data_type: VencDataTypeU,
    pub offset: u32,
    pub data_num: u32,
    pub pack_info: [VencPackInfo; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencStream {
    pub pst_pack: *mut VencPack,
    pub pack_count: u32,
    pub seq: u32,
    pub stream_info: [u32; 48],   
    pub advance_info: [u32; 128], 
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VencRcModeE {

    VencRcModeH264cbr = 1,
    VencRcModeH264vbr,
    VencRcModeH264avbr,
    VencRcModeH264fixqp,

    VencRcModeMjpegcbr,
    VencRcModeMjpegvbr,
    VencRcModeMjpegfixqp,

    VencRcModeH265cbr,
    VencRcModeH265vbr,
    VencRcModeH265avbr,
    VencRcModeH265fixqp,
    VencRcModeButt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VencRcAttrUnion {
    pub st_h264_cbr: VencH264Cbr,
    pub st_h264_vbr: VencH264Vbr,
    pub st_h264_avbr: VencH264Avbr,
    pub st_h264_fix_qp: VencH264FixQp,

    pub st_mjpeg_cbr: VencMjpegCbr,
    pub st_mjpeg_vbr: VencMjpegVbr,
    pub st_mjpeg_fix_qp: VencMjpegFixQp,

    pub st_h265_cbr: VencH265Cbr,
    pub st_h265_vbr:  VencH265Vbr,
    pub st_h265_avbr:  VencH265Avbr,
    pub st_h265_fix_qp:  VencH265FixQp,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencRcAttr {
    pub en_rc_mode: VencRcModeE,
    pub rc_attr_union: VencRcAttrUnion,
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH264Vbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub max_bit_rate: u32,
    pub min_bit_rate: u32,
    pub stat_time:u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH264Avbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub max_bit_rate: u32,
    pub min_bit_rate: u32,
    pub stat_time:u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH264FixQp {
    pub gop:u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub i_qp: u32,
    pub p_qp: u32,
    pub b_qp:u32
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH265Vbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub max_bit_rate: u32,
    pub min_bit_rate: u32,
    pub stat_time:u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH265Cbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub stat_time: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH265Avbr {
    pub gop: u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub max_bit_rate: u32,
    pub min_bit_rate: u32,
    pub stat_time:u32
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencH265FixQp {
    pub gop:u32,
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub i_qp: u32,
    pub p_qp: u32,
    pub b_qp:u32
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencMjpegCbr {
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub stat_time:u32
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencMjpegVbr {
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub bit_rate: u32,
    pub max_bit_rate: u32,
    pub min_bit_rate: u32,
    pub stat_time:u32
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VencMjpegFixQp {
    pub src_frame_rate_num: u32,
    pub src_frame_rate_den: u32,
    pub dst_frame_rate_num: u32,
    pub dst_frame_rate_den: u32,
    pub q_factor: u32,
} 

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum H264EProfile {
    H264eProfileBaseline           = 66,
    H264eProfileMain               = 77,
    H264eProfileHigh               = 100,
    H264eProfileButt
} 

//==================================RK MPI VENC==========================================

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RgnAttr {
    pub rgn_type_e: RgnTypeE,
    pub attr_union: RgnAttrUnion  
} 

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgnTypeE {
    OverlayRgn = 0,
    OverlayExRgn,
    CoverRgn,
    MosaicRgn,
    LineRgn,
    RgnButt
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub union RgnAttrUnion {
    pub overlay_attr: OverlayAttr ,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayAttr {
    pub pixel_format: PixelFormatE,
    pub size: SizeS,
    pub canvas_num: u32,
    pub clut_num: u32,
    pub clut: [u32; RGN_CLUT_NUM],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmap {
    pixel_format:PixelFormatE,
    widht: u32,
    height: u32,
    p_data: *mut std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RgnChnAttr {
    show: u32, //RkBool
    rgn_type: RgnTypeE,
    chn_attr_union: RgnChnAttrUnion
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub union RgnChnAttrUnion {
    overlay_chn: OverlayChnAttr,
    cover_chn: CoverChnAttr,
    mosaic_chn: MosaicChnAttr,
    line_chn: LineChnAttr
} 


#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayChnAttr {
    pub point: PointS,
    pub fg_alpha: u32,
    pub bg_alpha: u32,
    pub layer: u32, 
    pub qp_info: OverlayQpInfo,
    pub color_lut: [u32;RGN_COLOR_LUT_NUM],
    pub invert_color: OverlayInvertColor
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayQpInfo {
    pub enable: u32, //RkBool
    pub abs_qp: u32, //RkBool
    pub force_intra: u32, //RkBool
    pub qp: RkS32, 
} 

#[derive(Copy, Clone)]
#[repr(C)]
pub struct OverlayInvertColor {
    pub inv_col_area: SizeS,
    pub lum_thresh: u32,
    pub chg_mod: InvertColorModeE,
    pub inv_col_enable: u32, //RkBool
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvertColorModeE {
    LessthanLumThresh = 0,
    MorethanLumThresh,
    InverColorButt
}

#[repr(C)]
#[derive(Clone,Copy)]
pub struct CoverChnAttr {
    pub rect: RectS,
    pub color: u32,
    pub layer: u32,
    pub coordinate: RgnCoordinateE
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgnCoordinateE {
    RgbAbsCoor = 0,   
    RgnRatioCoor      
} 

#[repr(C)]
#[derive(Clone,Copy)]
pub struct MosaicChnAttr {
    pub rect: RectS,
    pub blk_size: MosaikBlkSizeE,
    pub layer: u32
} 

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosaikBlkSizeE {
    MosaicBlkSize8 = 0,    
    MosaicBlkSize16,       
    MosaicBlkSize32,       
    MosaicBlkSize64,       
    MosaicBlkSizeButt
} 

#[repr(C)]
#[derive(Clone,Copy)]
pub struct LineChnAttr {
    pub thick: u32,
    pub color: u32,
    pub start_point: PointS,
    pub end_point: PointS
}

#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct RgnCanvasInfo{
    pub canvas_blk:*mut libc::c_void, 
    pub addr: u64,
    pub size: SizeS,
    pub width: u32,
    pub height: u32,
    pub pixel_format: PixelFormatE
}