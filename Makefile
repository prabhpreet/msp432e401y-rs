SVDNAME=MSP432E401Y.svd

lib: 
	svd2rust -g -i svd/${SVDNAME}
	rm -rf src
	form -i lib.rs -o src/ && rm lib.rs	
	mv generic.rs src/
	cargo fmt
	
	