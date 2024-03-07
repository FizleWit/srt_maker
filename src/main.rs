fn main() {
    println!("Hello, world!");
}
//merge multiple files together
/*
ffmpeg -y -hide_banner -f concat -safe 0 -i list.txt -c copy out.mp4
write path relative to input.txt file as
file 'file_name' 
will concat in order from top to bottom
*/
//add srt files to .mp4 and re-write so they are in the same file 
/*
whisper .\out.mp4 --model medium.en --language English --output_format srt
specify model, input file, and language / outputfile as srt 
*/
//merge video with srt track
/*
ffmpeg -y -hide_banner -i .\out.mp4 -vf subtitles=out.mp4.srt output_srt.mp4
specify input, specify subtitle file, specify outputfile
*/
