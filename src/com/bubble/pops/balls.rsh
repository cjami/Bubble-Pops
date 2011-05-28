
typedef struct __attribute__((packed, aligned(4))) Ball {
    float2 delta;
    float2 position;
    //float4 color;
    float size;
    int active;
    int pointerId;
    int team;
    int scored;
    //int arcID;
    //float arcStr;
} Ball_t;
Ball_t *balls;


typedef struct BallControl {
    uint32_t dimX;
    rs_allocation ain;
    rs_allocation aout;
    float dt;
    int scores[2];
} BallControl_t;
