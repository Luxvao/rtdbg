fn init(script_id, script_store, global_store) {
	debug(script_id);

	// Edit the address to match the output string pointer
	let address = 0x55a3fd951ee0;
	
	linux::mprotect(address, 7, linux::PROT_WRITE | linux::PROT_READ);
	write_mem(address, "Hacked");
}
