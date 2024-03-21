use std::{env, io};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::result;
use std::process::Stdio;
fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let array = &args[1..];
    
    //let array = return_arguments(args);

    /*
Process!!
arguments = input video files
envoke tool 
    if arguments present
        if 1 argument
            assume want to create and add subtitles, ask for an outputfile name
        else if >1 argument present assume wants to merge together 
            check if pre-conditions for ffmpeg valid
            merdge videos , ask for output file name
            confirm if wants to add and merge subtitles,  run first case
        else no arguments present 
            for loop for all input files
            run else if case
*/
    if array.len() == 1 {
        println!("one arguemt create subtitles {} xxx", "Woop");

        create_subtitles_video(array[0].clone())
    }
    else if  array.len() > 1 {
        println!("more than one file merdging ");
        merdge_multiple_videos(array.to_vec());
        
    }
    else {
        println!("no arguemnts present entering console easy mode");
    }
}


fn create_subtitles_video(video_file: String){
    let whisper_srt_file = whisper_function(video_file.clone());
    merdge_srt_with_ffmpeg(video_file,whisper_srt_file);
}


fn whisper_function(video_file: String) -> String{
    println!("creating srt file from {} with whisper", video_file);
    return String::from("srt_file_path_name");
}

fn merdge_srt_with_ffmpeg(video_file: String, srt_file: String){
    println!("merdging srt file {} with video file {} ",srt_file, video_file);
}
fn merdge_multiple_videos(video_files: Vec<String>){
    println!("merdging multiple videos");
   //let mut temp = video_files.clone();
    //check_video_files(video_files)
    assert!(write_temp_file(video_files.clone()).is_ok());
    assert!(ffmpeg_command().is_ok());
    println!("write File");
    /*
    echo file '1.mp4' > output.txt & 
    echo file '2.mp4' >> output.txt & 
    echo file '3.mp4' >> output.txt && 
    ffmpeg -f concat -safe 0 -i output.txt -c copy output.mp4 
    && del output.txt
    */
}
fn ffmpeg_command() -> io::Result<()> {
    let mut ffmpeg_command_merge_video = Command::new("ffmpeg");
    ffmpeg_command_merge_video
    .arg("-y")
    .arg("-hide_banner")
    .arg("-f")
    .arg("concat")
    .arg("-safe")
    .arg("0")
    .arg("-i")
    .arg("temp.txt")
    .arg("-c")
    .arg("copy")
    .arg("output.mp4")
    .stdin(Stdio::null())
    .stdout(Stdio::null());
    ffmpeg_command_merge_video.spawn();
    Ok(())
}
fn write_temp_file(video_files: Vec<String>) -> io::Result<()> {
    let mut file = File::create("temp.txt")?;
    for el in video_files{
        file.write_all(format!("file '{}'\n", el).as_bytes())?;
    }
    Ok(())
   
}
fn check_video_files(video_files: Vec<String>){
    println!("video files safe for processing");
    //files must be of same streams (audio/video codecs, same base times) can be differnt types though, should have the same amount of video and audio streams

}
//merge multiple files together

//add srt files to .mp4 and re-write so they are in the same file 
/*
whisper .\out.mp4 --model medium.en --language English --output_format srt
specify model, input file, and language / outputfile as srt 
*/
//merge video with srt track
/*
ffmpeg -y -hide_banner -i .\out.mp4 -vf subtitles=out.mp4.srt output_srt.mp4 //re-encodes video
ffmpeg -y -hide_banner -i .\out.mp4 -i .\out.mp4.srt -c copy -c:s mov_text -metadata:s:s:0 language=eng outputsrt.mp4
specify input, specify subtitle file, specify outputfile
*/
/*
Process!!
arguments = input video files
envoke tool 
    if arguments present
        if 1 argument
            assume want to create and add subtitles, ask for an outputfile name
        else if >1 argument present assume wants to merge together 
            check if pre-conditions for ffmpeg valid
            merdge videos , ask for output file name
            confirm if wants to add and merge subtitles,  run first case
        else no arguments present 
            for loop for all input files
            run else if case
*/
/*
Problems
whisper is very slow. design some way to show progress bar and utilize gpu? bigger model? 
user has to preinstall ffmpeg and whisper, get environment variables
*/
