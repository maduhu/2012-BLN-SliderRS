// Copyright (C) 2011 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#pragma version(1)
#pragma rs java_package_name(de.devisnik.rs.drawer)


#include "rs_graphics.rsh"

#pragma stateVertex(parent)
#pragma stateStore(parent)

typedef struct __attribute__((packed, aligned(4))) Tile {
    float2 position;
    float2 destination;
    float2 size;
    int steps;
    rs_allocation texture;
} Tile_t;
Tile_t *tiles;

rs_program_fragment gProgramFragment;

// gTouchX and gTouchY are variables that will be reflected for use
// by the java API. We can use them to notify the script of touch events.
int gTouchX;
int gTouchY;

// This is invoked automatically when the script is created
void init() {
    gTouchX = 50.0f;
    gTouchY = 50.0f;
}


static void renderTile(uint32_t index) {
	Tile_t *tile = &tiles[index];
	if (tile->steps > 0) {
		// (float) (Math.cos((input + 1) * Math.PI) / 2.0f) + 0.5f;
		//-cos(angle * M_PI / 180) * RADIUS;
		//tile->position.x += (tile->destination.x - tile->position.x) / 50.f * ((cos((tile->steps/50.f+1)*M_PI) /2.f)+.5f);
		//tile->position.y += (tile->destination.y - tile->position.y) / 50.f * ((cos((tile->steps/50.f+1)*M_PI) /2.f)+.5f);
		tile->position.x += (tile->destination.x - tile->position.x) / tile->steps;
		tile->position.y += (tile->destination.y - tile->position.y) / tile->steps;
		tile->steps--;
	}
//	rsDebug("steps", tile->steps);
	rsgBindTexture(gProgramFragment, 0, tile->texture);
	rsgDrawRect(tile->position.x, tile->position.y, tile->position.x+tile->size.x, tile->position.y+tile->size.y, 0);	
}

static void updatePosition(uint32_t index) {
	Tile_t tile = tiles[index];
}

int root() {

    // Clear the background color
    rsgClearColor(0.0f, 0.0f, 0.0f, 0.0f);

	rsgBindProgramFragment(gProgramFragment);

    uint32_t dimX = rsAllocationGetDimX(rsGetAllocation(tiles));
    for (uint32_t ct=0; ct < dimX; ct++) {
		renderTile(ct);
    }
    return 1;
}
