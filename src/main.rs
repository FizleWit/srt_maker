fn main() {
    println!("Hello, world!");
}
//merge multiple files together
/*
files must be of same streams (audio/video codecs, same base times) can be differnt types though, should have the same amount of video and audio streams
ffmpeg -y -hide_banner -f concat -safe 0 -i list.txt -c copy out.mp4
write path relative to input.txt file as
file 'file_name' 
will concat in order from top to bottom
echo file '1.mp4' > output.txt & echo file '2.mp4' >> output.txt & echo file '3.mp4' >> output.txt && ffmpeg -f concat -safe 0 -i output.txt -c copy output.mp4 && del output.txt
*/
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
