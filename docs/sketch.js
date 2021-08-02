/*
 * Contour tracing library
 * https://github.com/STPR/contour_tracing
 *
 * Copyright (c) 2021, STPR - https://github.com/STPR
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

"use strict";

const TABLE = {w: 16, h: 9}, UPSCALE = 60;

const MN       = [[0, -1], [1, -1], [1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1]];
const O_VERTEX = [[-1, 0], [0, 0], [-1, -1], [0, 0], [0, -1], [0, 0], [0, 0]];
const H_VERTEX = [[0, 0], [0, 0], [-1, 0], [0, 0], [-1, -1], [0, 0], [0, -1]];
let   VERTEX   = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]];
const O_VALUE  = [1, 0, 2, 0, 4, 0, 8];
const H_VALUE  = [-4, 0, -8, 0, -1, 0, -2];
let   VALUE    = [0, 0, 0, 0, 0, 0, 0];
const TRACER   = [
				[0, 1, 0.5, 0, 1, 1, 0.5, 0.62],
				[0, 0, 0, 0, 0, 0, 0, 0],
				[0, 0, 1, 0.5, 0, 1, 0.38, 0.5],
				[0, 0, 0, 0, 0, 0, 0, 0],
				[1, 0, 0.5, 1, 0, 0, 0.5, 0.38],
				[0, 0, 0, 0, 0, 0, 0, 0],
				[1, 1, 0, 0.5, 1, 0, 0.62, 0.5]];
const PRESET_0 = [
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1, 0],
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
const PRESET_1 = [
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				[0,-1,-1,-1,-1,-1,-1,-1, 1, 1, 1, 1, 1, 1, 1, 0],
				[0,-1, 1, 1, 1,-1, 1,-1, 1,-1,-1,-1, 1,-1, 1, 0],
				[0,-1, 1, 1, 1,-1, 1,-1, 1,-1,-1,-1, 1,-1, 1, 0],
				[0,-1, 1, 1, 1,-1, 1,-1, 1,-1,-1,-1, 1,-1, 1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1, 1, 1, 1, 1, 1, 1, 1, 0],
				[0,-1, 1, 1, 1,-1, 1,-1, 1,-1,-1,-1, 1,-1, 1, 0],
				[0,-1,-1,-1,-1,-1,-1,-1, 1, 1, 1, 1, 1, 1, 1, 0],
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
const PRESET_2 = [
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				[0, 1, 1, 1, 1, 1, 1, 1,-1, 1, 1, 1, 1, 1, 1, 0],
				[0, 1,-1,-1,-1,-1,-1, 1,-1,-1,-1,-1,-1,-1, 1, 0],
				[0, 1,-1, 1, 1, 1,-1, 1,-1, 1, 1, 1, 1,-1, 1, 0],
				[0, 1,-1, 1,-1, 1,-1, 1,-1, 1,-1,-1, 1,-1, 1, 0],
				[0, 1,-1, 1, 1, 1,-1, 1,-1, 1,-1, 1, 1,-1, 1, 0],
				[0, 1,-1,-1,-1,-1,-1, 1,-1, 1,-1,-1,-1,-1, 1, 0],
				[0, 1, 1, 1, 1, 1, 1, 1,-1, 1, 1, 1, 1, 1, 1, 0],
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
const PRESET_3 = [
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				[0, 1, 1, 1, 1, 1,-1, 1, 1,-1, 1, 1, 1, 1, 1, 0],
				[0, 1,-1, 1, 1,-1, 1, 1, 1, 1,-1, 1, 1,-1, 1, 0],
				[0, 1, 1, 1,-1, 1, 1,-1,-1, 1, 1,-1, 1, 1, 1, 0],
				[0, 1, 1,-1, 1, 1,-1, 1, 1,-1, 1, 1,-1, 1, 1, 0],
				[0, 1,-1, 1, 1,-1, 1, 1, 1, 1,-1, 1, 1,-1, 1, 0],
				[0,-1, 1, 1,-1, 1, 1,-1,-1, 1, 1,-1, 1, 1,-1, 0],
				[0, 1, 1,-1, 1, 1, 1, 1, 1, 1, 1, 1,-1, 1, 1, 0],
				[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];

let cnv, i, v, rn, neighbors, viv = [0, 0, 0];
let c_table, c_index, c_data, preset, vision_step, closed;
let grid         = {x: 0, y: 0};
let cursor       = {x: 0, y: 0, ol: 0, hl: 0, running: false};
let tracer       = {x: 0, y: 0, running: false, o: [0, 0, 0, 0, 0, 0, 0]};
let mouse        = {x: 0, y: 0, color: 0};
let perlin_noise = {xoff: 0.0, yoff: 0.0, scale: 0.4}

let layer_grid;
let button_start;
let slider_fps;
let span_fps;
let button_reset;
let button_draw;
let button_clear;
let button_random;
let button_preset_1;
let button_preset_2;
let button_preset_3;
let started;
let reseted;
let drawing_mode_activated;
let done;

function setup() {
	pixelDensity(1);
	createCanvas(TABLE.w * UPSCALE, (TABLE.h + 1) * UPSCALE);
	layer_grid = createGraphics(TABLE.w * UPSCALE, (TABLE.h + 1) * UPSCALE);
	preset_1();

	button_start = createButton('START / STOP');
	slider_fps   = createSlider(1, 60, 5);
	slider_fps.style('width', '120px');
	span_fps = createSpan('5 fps ');
	span_fps.style('width:6ch;text-align:right;display:inline-block;white-space:pre;');
	button_reset = createButton('RESET');
	button_draw  = createButton('DRAW');
	button_draw.style('background', 'orange');
	button_clear = createButton('CLEAR');
	button_clear.style('background', 'orange');
	button_random   = createButton('RANDOM');
	button_preset_1 = createButton('PRESET 1');
	button_preset_2 = createButton('PRESET 2');
	button_preset_3 = createButton('PRESET 3');
	button_start.mousePressed(start);
	slider_fps.input(update_fps);
	button_reset.mousePressed(reset);
	button_draw.mousePressed(activate_drawing_mode);
	button_clear.mousePressed(clear_drawing_mode);
	button_random.mousePressed(random_noise);
	button_preset_1.mousePressed(preset_1);
	button_preset_2.mousePressed(preset_2);
	button_preset_3.mousePressed(preset_3);
}

function start() {
	if (started) {
		started = false;
		noLoop();
	  } else {
		drawing_mode_activated = false;
		reseted                = false;
		started                = true;
		loop();
	  }
}

function update_fps() {
	span_fps.html(slider_fps.value() + ' fps ');
}

function reset() {
	c_index = 0; c_data = []; c_data.push([null, null, null]); cursor = {x: 0, y: 1, ol: 0, hl: 0, running: true}; tracer.running = false; vision_step = 0; mouse.color = 0; done = false;
	if (preset == -1) {
		for (grid.y = 1; grid.y < TABLE.h - 1; grid.y++) {
			for (grid.x = 1; grid.x < TABLE.w - 1; grid.x++) {
				if (c_table[grid.y][grid.x] > 0) {
					c_table[grid.y][grid.x] = 1;
				}
				else {
					c_table[grid.y][grid.x] = -1;
				}
			}
		}
	}
	else if (preset == -2) {
		for (grid.y = 1; grid.y < TABLE.h - 1; grid.y++) {
			perlin_noise.xoff = 0.0;
			for (grid.x = 1; grid.x < TABLE.w - 1; grid.x++) {
				noiseDetail(4, 0.59);
				if   (noise(perlin_noise.xoff, perlin_noise.yoff) < 0.5) c_table[grid.y][grid.x]  = -1;
				else c_table[grid.y][grid.x]                                                      = 1;
				     perlin_noise.xoff                                                           += perlin_noise.scale;
			}
			perlin_noise.yoff += perlin_noise.scale;
		}
	}
	else if (preset == 0) c_table                        = JSON.parse(JSON.stringify(PRESET_0));
	else if (preset == 1) c_table                        = JSON.parse(JSON.stringify(PRESET_1));
	else if (preset == 2) c_table                        = JSON.parse(JSON.stringify(PRESET_2));
	else if (preset == 3) c_table                        = JSON.parse(JSON.stringify(PRESET_3));
	if   (drawing_mode_activated || preset == -2) preset = -1;
	   layer_grid.background(255);
	   layer_grid.noFill();
	   layer_grid.stroke('black');
	   layer_grid.strokeWeight(UPSCALE / 15);
	   layer_grid.rect(0, 0, TABLE.w * UPSCALE, (TABLE.h + 1) * UPSCALE);
	   layer_grid.strokeWeight(UPSCALE / 30);
	   layer_grid.line(0, TABLE.h * UPSCALE, TABLE.w * UPSCALE, TABLE.h * UPSCALE);
	   layer_grid.noStroke();
	   layer_grid.fill('darkgray'); 
	   for (grid.y = 1; grid.y < TABLE.h - 1; grid.y++) {
		   for (grid.x = 1; grid.x < TABLE.w - 1; grid.x++) {
			   if (c_table[grid.y][grid.x] > 0) {
				   layer_grid.rect(grid.x * UPSCALE, grid.y * UPSCALE, UPSCALE, UPSCALE);
			   }
		   }
		}
	started = false;
	reseted = true;
	loop();
}

function activate_drawing_mode() {
	drawing_mode_activated = true;
	preset                 = -1;
	reset();
}

function clear_drawing_mode() {
	drawing_mode_activated = true;
	preset                 = 0;
	reset();
}

function random_noise() {
	preset = -2;
	reset();
}

function preset_1() {
	preset = 1;
	reset();
}

function preset_2() {
	preset = 2;
	reset();
}

function preset_3() {
	preset = 3;
	reset();
}

function mousePressed() {
	mouse.x = constrain(int(mouseX / UPSCALE), 1, TABLE.w - 2);
	mouse.y = constrain(int(mouseY / UPSCALE), 1, TABLE.h - 2);
	if (mouse.x == int(mouseX / UPSCALE) && mouse.y == int(mouseY / UPSCALE)) {
		if (c_table[mouse.y][mouse.x] == 1) {
			mouse.color = 1;
		}
		else if (c_table[mouse.y][mouse.x] == -1) {
			mouse.color = -1;
		}
		else mouse.color = 0;
	}
}

function mouseReleased() {
		mouse.color = 0;
}

function draw() {
	if (started) { frameRate(slider_fps.value()); } else if (reseted) { frameRate(30); }
	function draw_grid() {
		image(layer_grid, 0, 0);
		fill('black'); noStroke(); textSize(UPSCALE / 2); textFont('Arial'); textAlign(CENTER, CENTER);
		for (grid.y = 1; grid.y < TABLE.h - 1; grid.y++) {
			for (grid.x = 1; grid.x < TABLE.w - 1; grid.x++) {
				if (c_table[grid.y][grid.x] != 1 && c_table[grid.y][grid.x] != -1) text(c_table[grid.y][grid.x], grid.x * UPSCALE + (UPSCALE / 2), grid.y * UPSCALE + (UPSCALE / 2));
			}
		}
		if (drawing_mode_activated) {
			stroke('bisque');
			strokeWeight(UPSCALE / 0.5);
			noFill();
			rect(0, 0, TABLE.w * UPSCALE, TABLE.h * UPSCALE);
			fill('black'); noStroke(); textSize(UPSCALE / 2); textFont('Arial'); textAlign(CENTER, CENTER);
			text('DRAWING MODE', (TABLE.w / 2) * UPSCALE, TABLE.h * UPSCALE + (UPSCALE / 2));
			layer_grid.noStroke();
			if (mouseIsPressed && mouse.color != 0) {
				mouse.x = constrain(int(mouseX / UPSCALE), 1, TABLE.w - 2);
				mouse.y = constrain(int(mouseY / UPSCALE), 1, TABLE.h - 2);
				if (mouse.color == 1) {
					c_table[mouse.y][mouse.x] = -1;
					layer_grid.fill('white');
				}
				else if (mouse.color == -1) {
					c_table[mouse.y][mouse.x] = 1;
					layer_grid.fill('darkgray');
				
				}
				layer_grid.rect(mouse.x  * UPSCALE , mouse.y * UPSCALE, UPSCALE, UPSCALE);
			}
		}
		else if (reseted) {
			fill('black'); noStroke(); textSize(UPSCALE / 2); textFont('Arial'); textAlign(CENTER, CENTER);
			text('PRESS START', (TABLE.w / 2) * UPSCALE, TABLE.h * UPSCALE + (UPSCALE / 2));
		}
		else if (done) {
			fill('black'); noStroke(); textSize(UPSCALE / 2); textFont('Arial'); textAlign(CENTER, CENTER);
			text('Done. PRESS RESET', (TABLE.w / 2) * UPSCALE, TABLE.h * UPSCALE + (UPSCALE / 2));
		}
		else {
			noFill(); stroke('black'); strokeWeight(UPSCALE / 8); rect(cursor.x * UPSCALE, cursor.y * UPSCALE, UPSCALE, UPSCALE);
			fill('black'); noStroke(); textSize(UPSCALE / 2); textFont('Courier New'); textAlign(CENTER, CENTER);
			text("Cursor's levels: outline = " + cursor.ol + ", hole = " + cursor.hl, (TABLE.w / 2) * UPSCALE, TABLE.h * UPSCALE + (UPSCALE / 2));
		}
		if (tracer.running) {
			noStroke();
			fill('rgba(0,0,0, 0.3)');
			beginShape();
			vertex((tracer.x + TRACER[tracer.o[0]][0]) * UPSCALE, (tracer.y + TRACER[tracer.o[0]][1]) * UPSCALE);
			vertex((tracer.x + TRACER[tracer.o[0]][2]) * UPSCALE, (tracer.y + TRACER[tracer.o[0]][3]) * UPSCALE);
			vertex((tracer.x + TRACER[tracer.o[0]][4]) * UPSCALE, (tracer.y + TRACER[tracer.o[0]][5]) * UPSCALE);
			vertex((tracer.x + TRACER[tracer.o[0]][6]) * UPSCALE, (tracer.y + TRACER[tracer.o[0]][7]) * UPSCALE);
			endShape(CLOSE);
			if (c_data[c_index][0]) {
				fill('blue');
				circle((tracer.x + O_VERTEX[tracer.o[0]][0] + 1) * UPSCALE, (tracer.y + O_VERTEX[tracer.o[0]][1] + 1) * UPSCALE, UPSCALE / 4);
			} else {
				fill('magenta');
				circle((tracer.x + H_VERTEX[tracer.o[0]][0] + 1) * UPSCALE, (tracer.y + H_VERTEX[tracer.o[0]][1] + 1) * UPSCALE, UPSCALE / 4);
			}
		}
		noFill(); strokeWeight(UPSCALE / 16);
		for (i = 1; i <= c_index; i++) {
			if (c_data[i][0] == true && c_data[i][1] > 0) {
				stroke('blue');
				beginShape();
				for (v = 2; v <= c_data[i][1] + 1; v++) {
					vertex((c_data[i][v][0] + 1) * UPSCALE, (c_data[i][v][1] + 1) * UPSCALE);
				}
				endShape();
			}
			else if (c_data[i][0] == false && c_data[i][1] > 0) {
				stroke('magenta');
				beginShape();
				for (v = 2; v <= c_data[i][1] + 1; v++) {
					vertex((c_data[i][v][0] + 1) * UPSCALE, (c_data[i][v][1] + 1) * UPSCALE);
				}
				endShape();
			}
		}
	}

	function vision() {
		if (vision_step == 0) draw_grid();
		vision_step += 1;

		if (vision_step == 1)  {
			neighbors = [c_table[tracer.y-1][tracer.x], c_table[tracer.y-1][tracer.x+1], c_table[tracer.y][tracer.x+1], c_table[tracer.y+1][tracer.x+1],
					c_table[tracer.y+1][tracer.x], c_table[tracer.y+1][tracer.x-1], c_table[tracer.y][tracer.x-1], c_table[tracer.y-1][tracer.x-1]];
			if (c_data[c_index][0]) {
				if (neighbors[tracer.o[7]] > 0 && neighbors[tracer.o[0]] > 0) {
					rn = 1;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[7]][0]) * UPSCALE, (tracer.y + MN[tracer.o[7]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[7]][0]) * UPSCALE, (tracer.y + MN[tracer.o[7]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			} else {
				if (neighbors[tracer.o[1]] < 0 && neighbors[tracer.o[0]] < 0) {
					rn = 1;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[1]][0]) * UPSCALE, (tracer.y + MN[tracer.o[1]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[1]][0]) * UPSCALE, (tracer.y + MN[tracer.o[1]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			}
		}

		else if (vision_step == 2)  {
			if (c_data[c_index][0]) {
				if (neighbors[tracer.o[0]] > 0) {
					rn = 2;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[0]][0]) * UPSCALE, (tracer.y + MN[tracer.o[0]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[0]][0]) * UPSCALE, (tracer.y + MN[tracer.o[0]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			} else {
				if (neighbors[tracer.o[0]] < 0) {
					rn = 2;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[0]][0]) * UPSCALE, (tracer.y + MN[tracer.o[0]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[0]][0]) * UPSCALE, (tracer.y + MN[tracer.o[0]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			}
		}

		else if (vision_step == 3)  {
			if (c_data[c_index][0]) {
				if (neighbors[tracer.o[1]] > 0 && neighbors[tracer.o[2]] > 0) {
					rn = 3;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[1]][0]) * UPSCALE, (tracer.y + MN[tracer.o[1]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[1]][0]) * UPSCALE, (tracer.y + MN[tracer.o[1]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			} else {
				if (neighbors[tracer.o[7]] < 0 && neighbors[tracer.o[6]] < 0) {
					rn = 3;
					fill('rgba(0,255,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[7]][0]) * UPSCALE, (tracer.y + MN[tracer.o[7]][1]) * UPSCALE, UPSCALE, UPSCALE);
					vision_step = 5;
				} else {
					fill('rgba(255,0,0, 0.3)'); noStroke(); rect((tracer.x + MN[tracer.o[7]][0]) * UPSCALE, (tracer.y + MN[tracer.o[7]][1]) * UPSCALE, UPSCALE, UPSCALE);
				}
			}
		}

		else if (vision_step == 4)  {
			rn          = 0;
			vision_step = 6;
		}

		if (vision_step == 6)  {
			vision_step = 0;
			return true;
		} else {
			return false;
		}
	}

	if (reseted && drawing_mode_activated) { draw_grid(); }
	else if (reseted) { draw_grid(); noLoop(); }
	else if (started && cursor.running) {
		cursor.x++;
		if (cursor.x == TABLE.w - 1) {
			cursor.ol = 0;
			cursor.hl = 0;
			cursor.x  = 1;
			cursor.y++;
			if (cursor.y == TABLE.h - 1) {
				done = true; started = false; cursor = {x: TABLE.w - 2, y: TABLE.h - 2, running: false}; noLoop();
			}
		}
		draw_grid();
		if (cursor.ol == cursor.hl && c_table[cursor.y][cursor.x] == 1) {
			tracer         = {x: cursor.x, y: cursor.y, running: true, o: [2, 3, 4, 5, 6, 7, 0, 1]};
			viv            = [7, 1, 0];
			VERTEX         = O_VERTEX;
			VALUE          = O_VALUE;
			cursor.running = false;
			vision_step    = 0;
			c_index++;
			c_data.push([true, 1, [cursor.x + VERTEX[tracer.o[0]][0], cursor.y + VERTEX[tracer.o[0]][1]]]);
		}
		else if (cursor.ol > cursor.hl && c_table[cursor.y][cursor.x] == -1) {
			tracer         = {x: cursor.x, y: cursor.y, running: true, o: [4, 5, 6, 7, 0, 1, 2, 3]};
			viv            = [1, 7, 6];
			VERTEX         = H_VERTEX;
			VALUE          = H_VALUE;
			cursor.running = false;
			vision_step    = 0;
			c_index++;
			c_data.push([false, 1, [cursor.x + VERTEX[tracer.o[0]][0], cursor.y + VERTEX[tracer.o[0]][1]]]);
		}
		if (cursor.running) {
			if (Math.abs(c_table[cursor.y][cursor.x]) == 2 || Math.abs(c_table[cursor.y][cursor.x]) == 4 || Math.abs(c_table[cursor.y][cursor.x]) == 10 || Math.abs(c_table[cursor.y][cursor.x]) == 12) {
				if (c_table[cursor.y][cursor.x] > 0) { cursor.ol += 1 } else { cursor.hl += 1 };
			}
			else if (Math.abs(c_table[cursor.y][cursor.x]) == 5 || Math.abs(c_table[cursor.y][cursor.x]) == 7 || Math.abs(c_table[cursor.y][cursor.x]) == 13 || Math.abs(c_table[cursor.y][cursor.x]) == 15) {
				if (c_table[cursor.y][cursor.x] > 0) { cursor.ol -= 1 } else { cursor.hl -= 1 };
			}
		}
	}
	else if (started && tracer.running) {
		if (vision()) {
			if (rn == 1) {
				c_table[tracer.y][tracer.x] += VALUE[tracer.o[0]];
				        tracer.x            += MN[tracer.o[viv[0]]][0];
				        tracer.y            += MN[tracer.o[viv[0]]][1];
				if (c_data[c_index][0]) {
					for(i = 2; i--;) tracer.o.unshift(tracer.o.pop()); // Rotate 90 degrees, counterclockwise
				}
				else {
					for(i = 2; i--;) tracer.o.push(tracer.o.shift()); // Rotate 90 degrees, clockwise
				}
				c_data[c_index][1]++;
				c_data[c_index].push([tracer.x + VERTEX[tracer.o[0]][0], tracer.y + VERTEX[tracer.o[0]][1]]);
			}
			else if (rn == 2) {
				c_table[tracer.y][tracer.x] += VALUE[tracer.o[0]];
				        tracer.x            += MN[tracer.o[0]][0];
				        tracer.y            += MN[tracer.o[0]][1];
			}
			else if (rn == 3) {
				c_table[tracer.y][tracer.x] += VALUE[tracer.o[0]];
				if (c_data[c_index][0]) {
					for(i = 2; i--;) tracer.o.push(tracer.o.shift());
				}
				else {
					for(i = 2; i--;) tracer.o.unshift(tracer.o.pop());
				}
				c_table[tracer.y][tracer.x] += VALUE[tracer.o[0]];
				c_data[c_index][1]++;
				c_data[c_index].push([tracer.x + VERTEX[tracer.o[0]][0], tracer.y + VERTEX[tracer.o[0]][1]]);
				if (c_data[c_index][0]) {
					for(i = 2; i--;) tracer.o.unshift(tracer.o.pop());
				}
				else {
					for(i = 2; i--;) tracer.o.push(tracer.o.shift());
				}
				tracer.x += MN[tracer.o[viv[1]]][0];
				tracer.y += MN[tracer.o[viv[1]]][1];
				c_data[c_index][1]++;
				c_data[c_index].push([tracer.x + VERTEX[tracer.o[0]][0], tracer.y + VERTEX[tracer.o[0]][1]]);
			}
			else {
				c_table[tracer.y][tracer.x] += VALUE[tracer.o[0]];
				if (c_data[c_index][0]) {
					for(i = 2; i--;) tracer.o.push(tracer.o.shift());
				}
				else {
					for(i = 2; i--;) tracer.o.unshift(tracer.o.pop());
				}
				c_data[c_index][1]++;
				c_data[c_index].push([tracer.x + VERTEX[tracer.o[0]][0], tracer.y + VERTEX[tracer.o[0]][1]]);
			}
			draw_grid();
		}
		if (tracer.x == cursor.x && tracer.y == cursor.y && c_data[c_index][1] > 2) {
			closed = false;
			while (!closed) {
				c_table[tracer.y][tracer.x]            += VALUE[tracer.o[0]];
				if      (tracer.o[0] == viv[2]) closed  = true;
				if (c_data[c_index][0]) {
					for(i = 2; i--;) tracer.o.push(tracer.o.shift());
				}
				else {
					for(i = 2; i--;) tracer.o.unshift(tracer.o.pop());
				}
				c_data[c_index][1]++;
				c_data[c_index].push([tracer.x + VERTEX[tracer.o[0]][0], tracer.y + VERTEX[tracer.o[0]][1]]);
				draw_grid();
			}
			if (Math.abs(c_table[cursor.y][cursor.x]) == 2 || Math.abs(c_table[cursor.y][cursor.x]) == 4 || Math.abs(c_table[cursor.y][cursor.x]) == 10 || Math.abs(c_table[cursor.y][cursor.x]) == 12) {
				if (c_table[cursor.y][cursor.x] > 0) { cursor.ol += 1 } else { cursor.hl += 1 };
			}
			else if (Math.abs(c_table[cursor.y][cursor.x]) == 5 || Math.abs(c_table[cursor.y][cursor.x]) == 7 || Math.abs(c_table[cursor.y][cursor.x]) == 13 || Math.abs(c_table[cursor.y][cursor.x]) == 15) {
				if (c_table[cursor.y][cursor.x] > 0) { cursor.ol -= 1 } else { cursor.hl -= 1 };
			}
			cursor.running = true; tracer.running = false;
		}
	}
}
