use crate::rkmpi::rk_struct::*;
use crate::rkmpi::rk_const::*;
unsafe extern "C" {
    
    //==================================RK MPI COMMON==========================================

    pub fn RK_MPI_SYS_Init() -> i32;
    pub fn RK_MPI_SYS_Exit() -> i32;
    pub fn RK_MPI_SYS_Bind(SrcChn: *const MppChnS, DestChn: *const MppChnS) -> i32;
    pub fn RK_MPI_SYS_UnBind(SrcChn: *const MppChnS, DestChn: *const MppChnS) -> i32;
    pub fn RK_MPI_SYS_MmzFlushCache(mb: *mut libc::c_void, clean: i32) -> i32;
    pub fn RK_MPI_SYS_CreateMB(
        mb: *mut *mut libc::c_void,  
        config: *const MbExtConfig,
    ) -> i32;

    pub fn RK_MPI_MB_CreatePool(config: *const MbPoolConfig) -> u32;
    pub fn RK_MPI_MB_GetMB(pool: u32, size: u64, block: i32) -> *mut libc::c_void;
    pub fn RK_MPI_MB_Handle2VirAddr(mb: *mut libc::c_void) -> *mut libc::c_void;
    pub fn RK_MPI_MB_ReleaseMB(mb: *mut libc::c_void) -> i32;
    pub fn RK_MPI_MB_GetSize(mb: *mut libc::c_void) -> i32;

    pub fn RK_MPI_CAL_COMM_GetPicBufferSize(
        attr: *const PicBufAttr,
        result: *mut MbPicCalResult,
    ) -> i32;

    //==================================RK MPI VI==========================================

    pub fn RK_MPI_VI_SetDevAttr(dev: i32, attr: *const ViDevAttr) -> i32;
    pub fn RK_MPI_VI_GetDevAttr(dev: i32, attr: *mut ViDevAttr) -> i32;
    pub fn RK_MPI_VI_EnableDev(dev: i32) -> i32;
    pub fn RK_MPI_VI_DisableDev(dev: i32) -> i32;
    pub fn RK_MPI_VI_CreatePipe(pipe: i32, pipe_attr:*const ViPipeAttrS);
    pub fn RK_MPI_VI_SetDevBindPipe(dev: i32, pipe: *const ViDevBindPipe) -> i32;
    pub fn RK_MPI_VI_SetChnAttr(pipe: i32, chn: i32, attr: *const ViChnAttr) -> i32;
    pub fn RK_MPI_VI_GetChnAttr(pipe: i32, chn: i32, attr: *mut ViChnAttr) -> i32;
    pub fn RK_MPI_VI_EnableChn(pipe: i32, chn: i32) -> i32;
    pub fn RK_MPI_VI_DisableChn(pipe: i32, chn: i32) -> i32;
    pub fn RK_MPI_VI_GetChnFrame(
        pipe: i32,
        chn: i32,
        frame: *mut VideoFrameInfo,
        timeout_ms: i32,
    ) -> i32;
    pub fn RK_MPI_VI_ReleaseChnFrame(
        pipe: i32,
        chn: i32,
        frame: *const VideoFrameInfo,
    ) -> i32;
    pub fn RK_MPI_VI_GetDevIsEnable(dev:i32) -> i32;

    //==================================RK MPI VENC==========================================

    pub fn RK_MPI_VENC_CreateChn(chn: i32, attr: *const VencChnAttr) -> i32;
    pub fn RK_MPI_VENC_SetChnParam(chn: i32, attr: *const VencChnParam) -> i32;
    pub fn RK_MPI_VENC_StartRecvFrame(chn: i32, param: *const VencRecvPicParam) -> i32;
    pub fn RK_MPI_VENC_SendFrame(chn: i32, frame: *const VideoFrameInfo, timeout_ms: i32) -> i32;
    pub fn RK_MPI_VENC_GetStream(chn: i32, stream: *mut VencStream, timeout_ms: i32) -> i32;
    pub fn RK_MPI_VENC_ReleaseStream(chn: i32, stream: *mut VencStream) -> i32;
    pub fn RK_MPI_VENC_SetJpegParam(chn: i32, venc_jpeg_param: *const VencJpegParams);
    pub fn RK_MPI_VENC_StopRecvFrame(chn: i32) -> i32;
	pub fn RK_MPI_VENC_DestroyChn(chn: i32) -> i32;

    //==================================RK MPI RGN==========================================
    pub fn RK_MPI_RGN_Create(handle:u32, rgn_attr: *const RgnAttr) -> i32;
    pub fn RK_MPI_RGN_Destroy(handle:u32) -> i32;
    pub fn RK_MPI_RGN_GetAttr(handle:u32,rgn_attr: *mut RgnAttr) -> i32;
    pub fn RK_MPI_RGN_SetAtty(handle:u32,rgn_attr: *const RgnAttr) -> i32;
    pub fn RK_MPI_RGN_SetBitMap(handle:u32,bitmap: *const Bitmap) -> i32;
    pub fn RK_MPI_RGN_AttachToChn(handle:u32,chn: *const MppChnS,rgn_chn_att: *const RgnChnAttr) -> i32;
    pub fn RK_MPI_RGN_DetachFromChn(handle:u32,chn:*const  MppChnS) -> i32;
    pub fn RK_MPI_RGN_GetCanvasInfo(handle:u32,canvas_info: *mut RgnCanvasInfo) -> i32;
    pub fn RK_MPI_RGN_UpdateCanvas(handle:u32) -> i32;

   
}