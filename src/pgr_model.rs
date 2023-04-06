

// // Make sure you add the `patch` crate to the `[dependencies]` key of your Cargo.toml file.
// use patch::Patch;

// let sample = "\
// --- before.py
// +++ path/to/after.py
// @@ -1,4 +1,4 @@
// -bacon
// -eggs
// -ham
// +python
// +eggy
// +hamster
//  guido\n";

// let patch = Patch::from_single(sample)?;
// assert_eq!(&patch.old.path, "before.py");
// assert_eq!(&patch.new.path, "path/to/after.py");

// // Print out the parsed patch file in its Rust representation
// println!("{:#?}", patch);

// // Print out the parsed patch file in the Unified Format. For input that was originally in the
// // Unified Format, this will produce output identical to that original input.
// println!("{}", patch); // use format!("{}\n", patch) to get this as a String