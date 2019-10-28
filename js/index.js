import("../pkg/virusdodge.js").then((game) => {
	import("../pkg/virusdodge_bg.wasm").then(wasm => {
		// Loads tileset
		const img = document.getElementById("tileset");
		var resourceCanvas = document.createElement("canvas");
		resourceCanvas.width = 32*4;
		resourceCanvas.height = 32;
		const resourceCtx = resourceCanvas.getContext('2d');
		resourceCtx.drawImage(img, 0, 0);
		const tileset = resourceCtx.getImageData(0, 0, 32*4, 32).data;

		console.log(tileset[0], tileset[1], tileset[2], tileset[3]);

		// Init game engine
		var world = game.World.new();
		const tiles_data = new Uint8ClampedArray(wasm.memory.buffer, world.tiles_data_ptr(), world.tiles_data_len());
		tiles_data.set(tileset);
		// Create engine context
		var ctx = document
			.getElementById("application-canvas")
			.getContext("2d");

		// Binds wasm memory buffer to a canvas' image data.
		const pixel_data = new Uint8ClampedArray(wasm.memory.buffer, world.pixel_data_ptr(), world.pixel_data_len());
		const imageData = new ImageData(pixel_data, 640, 360);

		var render = () => {
			// Updates the world and renders it into memory
			world.tick();
			world.render();

			// Renders memory into canvas context
			ctx.putImageData(imageData, 0, 0);

			requestAnimationFrame(render);
		}

		requestAnimationFrame(render);
	})
	.catch(console.error)
})
.catch(console.error)
