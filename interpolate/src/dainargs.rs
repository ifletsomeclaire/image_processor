use std::path::Path;

#[derive(Debug, Default)]
pub struct DainArgs {
    pub input: std::path::PathBuf,                 //Path to the input video
    pub output: std::path::PathBuf, //Output Path to generate the folder with all the files.
    pub output_name: String, //Name and extension of the videos that will be rendered. [mp4, webm, gif, apng]
    pub pallette: Option<bool>, //Generate a version of the file limiting the pallete.
    pub downscale: Option<i32>, //Downscale the input resolution. (-1 Turn off)
    pub anim_loop: Option<DainBool>, //1 if the animation do a perfect loop, default 0
    pub interpolations: Option<Interpolations>, //How much new frames will be created. 2, 4, 8
    pub downsample_fps: Option<i32>, //Cap fps to this value. (-1 Turn off)
    pub frame_handling: Option<i32>, //Interpolation Modes: Mode 1 - Default ; Mode 2 - Default, remove duplicates ; Mode 3 - Adaptative; Timestamp ; 4 - Static timestamp
    pub depth_awareness: Option<bool>, //Should depth be calculated in interpolations?
    pub split_size_x: Option<i32>, //How much division are made in the X axis of each frame. (-1 Turn off ; 1 = No divisions in this axis)
    pub split_size_y: Option<i32>, //How much division are made in the Y axis of each frame. (-1 Turn off ; 1 = No divisions in this axis)
    pub split_pad: Option<i32>, //Split frames using this values as pixel padding in width and height for each side of the frame.
    pub alpha: Option<i32>,     //Calculate transparency in interpolations. 0: Off, 1: Fast, 2: Slow
    pub check_scene_change: Option<i32>, //Sensitivity for scene change detection, skip interpolation if detect it as true. (-1 Turn off)
    pub audio_version: Option<i32>,      //Generate a version with audio.
    pub interpolation_algo: Option<i32>, //0: Default 1: Experimental
    pub interpolate_missing_original: Option<i32>, //Create interpolation of any missing files in the original_folders
    pub png_compress: Option<i32>,
}
impl DainArgs {
    pub fn to_arguments(&self) -> Vec<&str> {
        let mut result = vec![
            "--cli",
            "True",
            "--input",
            self.input
                .to_str()
                .expect("Expected Input Path; found invalid"),
            "--output",
            self.output
                .to_str()
                .expect("Expected Output Path; found invalid"),
            "--output_name",
            &self.output_name,
        ];
        if let Some(l) = self.anim_loop {
            result.push("--loop");
            match l {
                DainBool::OneTrue => {result.push("1")}
                DainBool::ZeroFalse => {result.push("0")}
            }
        }
        if let Some(i) = self.interpolations {
            result.push("--interpolations");
            match i {
                Interpolations::Two => {result.push("2")}
                Interpolations::Four => {result.push("4")}
                Interpolations::Eight => {result.push("8")}
            }
        }
        result.push("--png_compress");
        result.push("1");
        result
    }
    pub fn new<P: AsRef<Path>>(input: P, output: P, output_name: String) -> Self {
        Self {
            input: input.as_ref().to_path_buf(),
            output: output.as_ref().to_path_buf(),
            output_name: output_name,
            ..Default::default()
        }
    }
    pub fn with_interpolations(&mut self, count: Interpolations) {
        self.interpolations = Some(count)
    }
    pub fn with_loop(&mut self, count: DainBool) {
        self.anim_loop = Some(count)
    }
}

#[derive(Debug)]
pub enum FileType {
    Gif,
    Mp4,
    Webm,
    Apng,
}
#[derive(Debug, Copy, Clone)]
pub enum DainBool {
    OneTrue,
    ZeroFalse,
}
#[derive(Debug, Copy, Clone)]
pub enum Interpolations {
    Two,
    Four,
    Eight
}
