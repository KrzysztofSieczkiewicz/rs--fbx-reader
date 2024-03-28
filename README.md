# fbx_reader_rs

Basic reader for .fbx format files. This is meant to be just practical, first attempt at rust programming
After specifying path to fbx file, code should verify if it's valid file and print it's structure.
It's entirely possible to extend this to also write and edit fbx files with multiple properties, but due to existence of fbx SKD  this seems redundant at best.

Created as CLI, example run:
cargo run examples/sickle.fbx
Works with both absolute and relative paths

To understand fbx file formatting and inner working I suggest reading:
https://code.blender.org/2013/08/fbx-binary-file-format-specification/
https://web.archive.org/web/20160605023014/https://wiki.blender.org/index.php/User:Mont29/Foundation/FBX_File_Structure
and partially SDK documentation
http://docs.autodesk.com/FBX/2014/ENU/FBX-SDK-Documentation/index.html

-- IMPORTANT --
To be properly read, objects must not have any materials assigned as they are written in slightly different way. And there was no point in extending this test further
