mod video;
mod rkmpi;
use rkmpi::rk_codec::RkCodecIdE;
use std::thread;
use std::time::Duration;
use rkmpi::{rk_const::*,rk_fn::*,rk_struct::*};
use libc::{c_int,c_void};   
use std::mem::MaybeUninit;
use std::net::TcpStream;
use std::io::prelude::*;
use video::*;

fn main(){
		unsafe{get_cam_stream()};
		// unsafe { get_venc_jpeg();}
		loop{
		}
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
		
		//делаю RGN OSD, пример: https://github.com/LuckfoxTECH/luckfox-pico/blob/main/media/samples/simple_test/simple_vi_bind_venc_osd.c
		
		
	}


 

 