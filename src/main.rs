use lf_pico_rust::rkmpi::rk_codec::RkCodecIdE;
use lf_pico_rust::rkmpi::rk_struct::VencRcModeE::VencRcModeH264cbr;
use lf_pico_rust::rkmpi::rk_struct::VencRcModeE::VencRcModeMjpegcbr;
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::FourCC;
use v4l::video::{Capture,CaptureMplane};
use std::thread;
use std::time;
use std::time::Duration;
use lf_pico_rust::rkmpi::{rk_const::*,rk_fn::*,rk_struct::*};
use libc::{c_int,c_void};   
use v4l::v4l2;
use v4l::v4l_sys::*;
use std::mem::MaybeUninit;
use std::net::TcpStream;
use std::io::prelude::*;
pub unsafe fn init_vi_dev(dev_id:i32){
	unsafe{
		let mut dev_attr = std::mem::zeroed::<ViDevAttr>();
		let ret = RK_MPI_VI_GetDevAttr(dev_id, &mut dev_attr);
		if ret != 0 {
			RK_MPI_VI_SetDevAttr(dev_id, &dev_attr);
		}
		else{
			println!("DEV ALREADY SET ATTR!!!")
		}
		let ret = RK_MPI_VI_GetDevIsEnable(dev_id);
		if ret != 0 {
			let pipe_id = dev_id as u32;
			let pipe_attr = std::mem::zeroed::<ViPipeAttrS>();
			//RK_MPI_VI_CreatePipe(pipe_id as i32, &pipe_attr);
			RK_MPI_VI_EnableDev(dev_id);
			let mut pipe = std::mem::zeroed::<ViDevBindPipe>();
			pipe.u32_num = 1;
			pipe.pipe_id[0] = pipe_id;
			RK_MPI_VI_SetDevBindPipe(dev_id, &pipe);
		}
		else{
			println!("DEV ALREADY ENABLE!!!")
		}
		
	}
	println!("RK MPI VI DEV SUCCES INIT!!!");
}

pub unsafe fn init_vi_chn(channel_id:i32,width:u32,height:u32){
	unsafe{
		let mut buf_cnt = 4; // повысил с 2 до 4
		let mut chn_attr = std::mem::zeroed::<ViChnAttr>();
		chn_attr.st_isp_opt.u32_buf_count = buf_cnt;
		chn_attr.st_isp_opt.en_memory_type = VI_V4L2_MEMORY_TYPE_DMABUF;
		chn_attr.st_size.u32_width = width;
		chn_attr.st_size.u32_height = height;
		chn_attr.en_pixel_format = RK_FMT_YUV420SP;
		chn_attr.en_compress_mode = COMPRESS_MODE_NONE;
		chn_attr.st_frame_rate = FrameRateCtrlS { 
			s32_dst_frm_rate: 25, 
			s32_src_frm_rate: 25 
		};
		chn_attr.u32_depth = 0;   
		chn_attr.en_alloc_buf_type = 0; 
		chn_attr.st_isp_opt.b_no_use_lib_v4l2 = 0;
		chn_attr.st_isp_opt.a_entity_name = [0; 32]; 
		chn_attr.st_isp_opt.en_capture_type = 0;
		let mut ret = RK_MPI_VI_SetChnAttr(0, channel_id, &chn_attr);
		ret += RK_MPI_VI_EnableChn(0, channel_id);   
		if ret !=0{
			println!("Enable Chn RET: {}",ret);
		}   
	}
	println!("RK MPI VI CHN SUCCES INIT!!!");
}

pub unsafe fn init_venc(channel_id:i32,width:u32,height:u32,rk_codec_id:RkCodecIdE){
	unsafe{
		let mut venc_chn_attr: MaybeUninit<VencChnAttr> = MaybeUninit::uninit();
		let mut venc_chn_param: MaybeUninit<VencChnParam> = MaybeUninit::uninit();

		std::ptr::write_bytes(venc_chn_attr.as_mut_ptr(), 0, 1);
		std::ptr::write_bytes(venc_chn_param.as_mut_ptr(), 0, 1);

		let venc_chn_attr = &mut *venc_chn_attr.as_mut_ptr();
		let mut venc_chn_param =  &mut *venc_chn_param.as_mut_ptr();

		venc_chn_attr.venc_attr.enc_type = rk_codec_id;
		venc_chn_attr.venc_attr.pixel_format = PixelFormatE::PixelFormatYuvSemiplanar420;
		venc_chn_attr.venc_attr.pic_width = width;
		venc_chn_attr.venc_attr.pic_height = height;
		venc_chn_attr.venc_attr.vir_width = width;
		venc_chn_attr.venc_attr.vir_height = height;
		venc_chn_attr.venc_attr.stream_buf_cnt = 2;
		venc_chn_attr.venc_attr.buf_size = width * height * 3 / 2;
		venc_chn_attr.venc_attr.mirror =  MirrorTypeE::MirrorNone;
		match rk_codec_id{
			RkCodecIdE::RkVideoIdJpeg => {
				venc_chn_attr.venc_attr.venc_union.jpege.support_dcf = 0; //RK_FALSE
				venc_chn_attr.venc_attr.venc_union.jpege.mpf_cfg.large_thumb_nail_num = 0;
				venc_chn_attr.venc_attr.venc_union.jpege.receive_mode = VencPicReceiveModeE::VencPicReceiveSingle;
			},
			RkCodecIdE::RkVideoIdMjpeg => {
				venc_chn_attr.rc_attr.en_rc_mode = VencRcModeMjpegcbr;
				venc_chn_attr.rc_attr.rc_attr_union.st_mjpeg_cbr.bit_rate = 10 *1024;
			},
			RkCodecIdE::RkVideoIdAvc => {
				venc_chn_attr.rc_attr.en_rc_mode = VencRcModeH264cbr;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.gop = 60;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.src_frame_rate_num = 25;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.src_frame_rate_den = 25;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.dst_frame_rate_num = 25;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.dst_frame_rate_den = 2;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.bit_rate = 10 * 1024;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.stat_time = 3;
				venc_chn_attr.venc_attr.profile = 100;
			}
			RkCodecIdE::RkVideoIdHevc => {
				venc_chn_attr.rc_attr.en_rc_mode = VencRcModeE::VencRcModeH265cbr;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.gop = 60;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.src_frame_rate_num = 25;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.src_frame_rate_den = 1;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.dst_frame_rate_num = 25;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.dst_frame_rate_den = 1;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.bit_rate = 10 * 1024;
				venc_chn_attr.rc_attr.rc_attr_union.st_h264_cbr.stat_time = 3;
				venc_chn_attr.venc_attr.profile = 100;
			}
			_ => {}
		}

		RK_MPI_VENC_CreateChn(channel_id, venc_chn_attr);

		venc_chn_param.frame_rate.enable = 0; //RK_FALSE
		venc_chn_param.frame_rate.src_frm_rate_num = 25;
		venc_chn_param.frame_rate.src_frm_rate_den = 1;
		venc_chn_param.frame_rate.dst_frm_rate_num = 10;
		venc_chn_param.frame_rate.dst_frm_rate_den = 1;

		RK_MPI_VENC_SetChnParam(channel_id, venc_chn_param);

		let mut venc_recv_param = std::mem::zeroed::<VencRecvPicParam>();
		venc_recv_param.recv_pic_num = -1;
		RK_MPI_VENC_StartRecvFrame(0, &venc_recv_param);
	}
}

pub unsafe fn get_raw_frame(){
	unsafe{
		let mut vi_frame_info = std::mem::zeroed::<VideoFrameInfo>();
		loop{
			let ret = RK_MPI_VI_GetChnFrame(0, 0, &mut vi_frame_info, 1000);
			if ret == 0 {
				println!("RES WIDTH {:?}",vi_frame_info.vi_frame.width);
				let adr = RK_MPI_MB_Handle2VirAddr(vi_frame_info.vi_frame.p_mb_blk) as *const u8;
				if adr.is_null(){
					println!("Error! Null adress!");
				}else{
					let data_size = (vi_frame_info.vi_frame.width * vi_frame_info.vi_frame.height * 3 / 2) as usize;
					let data_slice = std::slice::from_raw_parts(adr,data_size);
					let filename = format!("frame_{}.nv12", vi_frame_info.vi_frame.pts);
					if let Err(e) = std::fs::write(&filename,data_slice){
						println!("Failed to save data!");
					}
					else{
						println!("Save data to {}",filename);
					}
				}
				RK_MPI_VI_ReleaseChnFrame(0, 0, &vi_frame_info);
			}
			println!("RES {}",ret);
		}
	}
}

pub unsafe fn get_venc_jpeg(){
	let mut ret: c_int;
	unsafe{
		ret = RK_MPI_SYS_Init();

		init_vi_dev(0);
		init_vi_chn(0,  1920, 1080);

		init_venc(0,1920,1080,RkCodecIdE::RkVideoIdJpeg);

		let src_chn = MppChnS{en_mod_id: RkModIdE::RkIdVi,
										s32_chn_id: 0,
										s32_dev_id:0
										};
		let dest_chn =  MppChnS{en_mod_id: RkModIdE::RkIdVenc,
										s32_chn_id: 0,
										s32_dev_id:0
										};
		ret = RK_MPI_SYS_Bind(&src_chn,&dest_chn);

		let mut jpeg_param = std::mem::zeroed::<VencJpegParams>();
		jpeg_param.q_factor = 77;
		RK_MPI_VENC_SetJpegParam(0,&jpeg_param);


		std::thread::sleep(Duration::from_millis(30));

		let mut frame = std::mem::zeroed::<VencStream>();
		let pack: Box<VencPack> = Box::new(unsafe { MaybeUninit::zeroed().assume_init() });
		frame.pst_pack = Box::into_raw(pack);
										
		ret = RK_MPI_VENC_GetStream(0, &mut frame, 500);
		if ret == 0{
			let adr = RK_MPI_MB_Handle2VirAddr((*frame.pst_pack).p_mb_blk) as *const u8;
			let data_size = (*frame.pst_pack).len as usize;
			let data_slice = std::slice::from_raw_parts(adr,data_size);
			
			let filename = "frame.jpeg";
			if let Err(e) = std::fs::write(&filename,data_slice){
					println!("Failed to save data!");
				}
				else{
					println!("Save data to {}",filename);
				}

			RK_MPI_VENC_ReleaseStream(0, &mut frame);
		}

		unsafe { drop(Box::from_raw(frame.pst_pack)); }
		

		ret = RK_MPI_SYS_UnBind(&src_chn,&dest_chn);
		if ret != 0{
			println!("RK MPI SYS UnBind Fail with RET: {}", ret);
		}

		RK_MPI_VI_DisableChn(0, 0);

		RK_MPI_VENC_StopRecvFrame(0);
		RK_MPI_VENC_DestroyChn(0);

		RK_MPI_VI_DisableDev(0);
		RK_MPI_SYS_Exit();
	}
}

unsafe fn get_cam_stream(){
	let heandle = thread::Builder::new().name("MJPEG STREAME".to_string()).spawn(move || {
		unsafe{
			let mut stream = TcpStream::connect("172.32.0.94:8080").unwrap();
			let mut ret;
			ret = RK_MPI_SYS_Init();

			init_vi_dev(0);
			init_vi_chn(0,  1920, 1080);
			
			init_venc(0,1920,1080,RkCodecIdE::RkVideoIdMjpeg);

			let src_chn = MppChnS{en_mod_id: RkModIdE::RkIdVi,
											s32_chn_id: 0,
											s32_dev_id:0
											};
			let dest_chn =  MppChnS{en_mod_id: RkModIdE::RkIdVenc,
											s32_chn_id: 0,
											s32_dev_id:0
											};

			let mut jpeg_param = std::mem::zeroed::<VencJpegParams>();
			jpeg_param.q_factor = 77;
			RK_MPI_VENC_SetJpegParam(0,&jpeg_param);

			ret = RK_MPI_SYS_Bind(&src_chn,&dest_chn);

			loop{
				let mut frame = std::mem::zeroed::<VencStream>();
				let pack: Box<VencPack> = Box::new(unsafe { MaybeUninit::zeroed().assume_init() });
				frame.pst_pack = Box::into_raw(pack);
				ret = RK_MPI_VENC_GetStream(0, &mut frame, 1000);
				if ret == 0{
					let adr = RK_MPI_MB_Handle2VirAddr((*frame.pst_pack).p_mb_blk) as *const u8;
					let data_size = (*frame.pst_pack).len as usize;
					let data_slice = std::slice::from_raw_parts(adr,data_size);
					stream.write(data_slice);

					RK_MPI_VENC_ReleaseStream(0, &mut frame);
				}
				else {
					println!("FRAME RECV RET ERR = {} ",ret);
				}
				unsafe { drop(Box::from_raw(frame.pst_pack)); }
				thread::sleep(Duration::from_millis(30));
			}
			
			ret = RK_MPI_SYS_UnBind(&src_chn,&dest_chn);
			if ret != 0{
				println!("RK MPI SYS UnBind Fail with RET: {}", ret);
			}

			RK_MPI_VI_DisableChn(0, 0);

			RK_MPI_VENC_StopRecvFrame(0);
			RK_MPI_VENC_DestroyChn(0);

			RK_MPI_VI_DisableDev(0);
			RK_MPI_SYS_Exit();

	
		}
	});
}



fn main(){
		unsafe{get_cam_stream()};
		loop{}
		// RkLunch-stop.sh
		// rkaiq_3A_server &
		
		// VENC jpeg работает 
		// сделать стриминг по Ethernet

		//пример venc jpeg https://github.com/LuckfoxTECH/luckfox-pico/blob/main/media/samples/simple_test/simple_vi_bind_venc_jpeg.c

		//пример venc rtsp (h264 в том числе) https://github.com/LuckfoxTECH/luckfox-pico/blob/main/media/samples/simple_test/simple_vi_bind_venc_rtsp.c#L115

		//дописываю venc_init из h264 примера 
		// остановился на venc_init (вроде все написал там)
		//venc h264(h265) не работает
		//запись кадров от jpeg в большом колличестве работает затестил, предварительно увеличив буффер vi с 2 до 4
		//venc так и не поборол потому что почему-то не привязывается pipe\

		//можно попробовать стримить jpeg по std::tcp и все | сделал, норм работает, прикольно
		
}


 