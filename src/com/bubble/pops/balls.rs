#pragma version(1)
#pragma rs java_package_name(com.bubble.pops)
#include "rs_graphics.rsh"

#include "balls.rsh"

#pragma stateVertex(parent)
#pragma stateStore(parent)

rs_program_fragment gPFPoints;
rs_program_fragment gPFLines;
rs_mesh partMesh;

typedef struct __attribute__((packed, aligned(4))) Point {
    float2 position;
    float size;
    uchar4 color;
} Point_t;
Point_t *point;

typedef struct VpConsts {
    rs_matrix4x4 MVP;
} VpConsts_t;
VpConsts_t *vpConstants;

rs_script physics_script;

Ball_t *balls1;
Ball_t *balls2;

static int frame = 0;
static float4 color1 = {0.f, 1.f, 0.f, 1.f};
static float4 color2 = {0.f, 0.f, 1.f, 1.f};

void initParts(int w, int h)
{
    uint32_t dimX = rsAllocationGetDimX(rsGetAllocation(balls1));

    for (uint32_t ct=0; ct < dimX; ct++) {
        balls1[ct].delta.x = 0.f;
        balls1[ct].delta.y = 0.f;
        balls1[ct].size = 10.f;
        balls1[ct].active = 1;
        balls1[ct].pointerId = -1;
        balls1[ct].team = ct % 2;
	    balls1[ct].position.y = rsRand(0.f, (float)h);
        if(balls1[ct].team){
	        balls1[ct].position.x = rsRand(0.f, 0.5f*(float)w);
        }else{
	        balls1[ct].position.x = rsRand(0.f, 0.5f*(float)w) + 0.5f*(float)w;
        }
        rsDebug("Ball created. Team: ", balls1[ct].team);
    }
}



int root() {
    rsgClearColor(0.f, 0.f, 0.f, 1.f);

    BallControl_t bc;
    Ball_t *bout;

    if (frame & 1) {
        rsSetObject(&bc.ain, rsGetAllocation(balls2));
        rsSetObject(&bc.aout, rsGetAllocation(balls1));
        bout = balls2;
    } else {
        rsSetObject(&bc.ain, rsGetAllocation(balls1));
        rsSetObject(&bc.aout, rsGetAllocation(balls2));
        bout = balls1;
    }

    bc.dimX = rsAllocationGetDimX(bc.ain);
    bc.dt = 1.f / 30.f;

    rsForEach(physics_script, bc.ain, bc.aout, &bc);

    for (uint32_t ct=0; ct < bc.dimX; ct++) {
    	if(bout[ct].active){
        	point[ct].position = bout[ct].position;        	
        	point[ct].size = 6.f /*+ bout[ct].color.g * 6.f*/ * bout[ct].size;
        	if (bout[ct].team) {
        		point[ct].color = rsPackColorTo8888(color1);
        	} else {
        		point[ct].color = rsPackColorTo8888(color2);
        	}
        }else{
        	// Don't draw inactive balls
        	point[ct].size = 0.f;
        }
    }

    frame++;
    rsgBindProgramFragment(gPFPoints);
    rsgDrawMesh(partMesh);
    rsClearObject(&bc.ain);
    rsClearObject(&bc.aout);
    return 10;
}

