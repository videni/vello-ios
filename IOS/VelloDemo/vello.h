
#ifndef VELLO_DEMO
#define VELLO_DEMO

#include <stdint.h>

// AppSurface
typedef struct {
    void *view;
    void *metal_layer;  // CAMetalLayer
    int32_t maximum_frames;
    void (*callback_to_swift)(int32_t arg);
} IOSViewObj;

typedef struct Affine
{
  float a;
  float b;
  float c;
  float d;
  float tx;
  float ty;
} Affine;

typedef struct Rectangle
{
  float x;
  float y;
  float width;
  float height;
} Rectangle;

typedef struct VelloApp VelloApp;

VelloApp* App_create(IOSViewObj ios_obj, const char *path);

void App_render(VelloApp *vello_app, uint32_t scene_idx, Rectangle bounds, float scaler_factor, Affine transform);


typedef struct Array
{
  void *ptr;
  int32_t len;
  int32_t cap;
} Array;


Array* scenes();

#endif
