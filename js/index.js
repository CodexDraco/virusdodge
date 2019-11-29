const KEY_W = 1;
const KEY_A = 2;
const KEY_S = 4;
const KEY_D = 8;

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

		// Init game engine
		var world = game.World.new();
		const tiles_data = new Uint8ClampedArray(wasm.memory.buffer, world.tiles_data_ptr(), world.tiles_data_len());
		tiles_data.set(tileset);

		// Create engine context
		var canvas = document.getElementById("application-canvas")
		var ctx = canvas.getContext("2d");

		// Setups event listeners
		document.addEventListener('keydown', event => {
			if(!event.repeat) {
				if(event.key == "w" || event.key == "W") {
					world.keydown(KEY_W);
				} else if(event.key == "a" || event.key == "A") {
					world.keydown(KEY_A);
				} else if(event.key == "s" || event.key == "S") {
					world.keydown(KEY_S);
				} else if(event.key == "d" || event.key == "D") {
					world.keydown(KEY_D);
				}
			}
		});
		document.addEventListener('keyup', event => {
			if(event.key == "w" || event.key == "W") {
				world.keyup(KEY_W);
			} else if(event.key == "a" || event.key == "A") {
				world.keyup(KEY_A);
			} else if(event.key == "s" || event.key == "S") {
				world.keyup(KEY_S);
			} else if(event.key == "d" || event.key == "D") {
				world.keyup(KEY_D);
			}
		});
		document
			.getElementById('options-button')
			.addEventListener('click', event => {
				// Show options form
				document.getElementById('application-options')
					.setAttribute('class', 'visible');
			});
		document
			.getElementById('close-options-button')
			.addEventListener('click', event => {
				// Show options form
				document.getElementById('application-options')
					.setAttribute('class', 'hidden');
			});
		document
			.getElementById('fullscreen-button')
			.addEventListener('click', event => {
			if(canvas.requestFullScreen) {
				canvas.requestFullScreen();
			} else if(canvas.webkitRequestFullScreen) {
				canvas.webkitRequestFullScreen();
			} else if(canvas.mozRequestFullScreen) {
				canvas.mozRequestFullScreen();
			}
		});

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
