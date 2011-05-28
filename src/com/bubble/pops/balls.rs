#pragma version(1)
#pragma rs java_package_name(com.bubble.pops)
#include "rs_graphics.rsh"

#include "balls.rsh"

#pragma stateVertex(parent)
#pragma stateStore(parent)

static const int RESET_EVENT = 2;

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

int ws;
int hs;

void initParts(int w, int h)
{
    uint32_t dimX = rsAllocationGetDimX(rsGetAllocation(balls1));

    for (uint32_t ct=0; ct < dimX; ct++) {
        balls1[ct].delta.x = 0.f;
        balls1[ct].delta.y = 0.f;
        balls1[ct].size = 20.f;
        balls1[ct].active = 1;
        balls1[ct].pointerId = -1;
        balls1[ct].team = ct % 2;
	    balls1[ct].position.y = ((ct + 1) / 2) * 150.f;
        if(balls1[ct].team){
	        balls1[ct].position.x = rsRand(0.f, 0.3f*(float)w);
        }else{
	        balls1[ct].position.x = rsRand(0.f, 0.3f*(float)w) + 0.7f*(float)w;
        }
        rsDebug("Ball created. Team: ", balls1[ct].team);
    }
    
    hs = h;
    ws = w;
}

//char* massiveHack(int num) {
//	static char chars[10] = "0123456789";
//	static char thisstr[10];
//	// thisstr[6] = "     "; // null terminated string
//	int i = 0;
//	while (num/10 > 1 && i < 6) {
//		thisstr[i] = chars[num % 10];
//		num = num / 10;
//		i++;
//	}
//	thisstr[i] = '\0';
//	return thisstr;
//}


// The official libc "itoa" code
char *itoa(i)
     int i;
{
	int INT_DIGITS = 19;
  /* Room for INT_DIGITS digits, - and '\0' */
  static char buf[21];
  char *p = buf + INT_DIGITS + 1;	/* points to terminating '\0' */
  if (i >= 0) {
    do {
      *--p = '0' + (i % 10);
      i /= 10;
    } while (i != 0);
    return p;
  }
  else {			/* i < 0 */
    do {
      *--p = '0' - (i % 10);
      i /= 10;
    } while (i != 0);
    *--p = '-';
  }
  return p;
}


int root() {
    rsgClearColor(0.05f, 0.05f, 0.05f, 0.05f);

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
    bc.dt = 1.f / 10.f;

    rsForEach(physics_script, bc.ain, bc.aout, &bc);

	int active_count = 0;

    for (uint32_t ct=0; ct < bc.dimX; ct++) {
    	if(bout[ct].active){
    		active_count++;
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
    char buf[64];
    rsgFontColor(0.f, 0.f, 1.f, 1.f);
	rsgDrawText("Team Blue: ", 520, 30);
    rsgDrawText(itoa(bc.scores[0]), 670, 30);
    rsgFontColor(0.f, 1.f, 0.f, 1.f);
    rsgDrawText("Team Green: ", 520, 680);
    rsgDrawText(itoa(bc.scores[1]), 670, 680);
    rsgBindProgramFragment(gPFPoints);
    rsgDrawMesh(partMesh);
    rsClearObject(&bc.ain);
    rsClearObject(&bc.aout);
    
    if (active_count == 0) {
    	rsSendToClient(RESET_EVENT);
    	initParts(ws,hs);
    }
    
    return 15;
}

