# deepseek-coder-v2:236b / 235.7B

import bpy
import random

# Clear existing objects
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)

# Set up the scene
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 500

# Create stars in the background
def create_stars(num_stars):
    for i in range(num_stars):
        x = random.uniform(-10, 10)
        y = random.uniform(-10, 10)
        z = random.uniform(-10, -50)
        bpy.ops.mesh.primitive_uv_sphere_add(radius=0.02, location=(x, y, z))
        star = bpy.context.object
        star.data.materials.append(bpy.data.materials.new(name="StarMaterial"))
        star.active_material.diffuse_color = (1, 1, 1, 1)

create_stars(500)

# Create scrolling text
text_curve = bpy.ops.object.text_add(location=(0, 0, -20))
text_obj = bpy.context.object
text_obj.data.body = "A long time ago in a galaxy far,\nfar away....\nthere was man who felt...\nsome pain in his lower stomach..."
text_obj.data.font = bpy.data.fonts.load("/System/Library/Fonts/Supplemental/AppleMyungjo.ttf")  # Adjust path as needed
text_obj.data.size = 1.5
text_obj.data.align_x = 'CENTER'
text_obj.data.align_y = 'CENTER'
text_obj.data.extrude = 0.02

# Set initial width and color
text_obj.scale[0] = 0.8  # Width at the bottom
text_obj.active_material = bpy.data.materials.new(name="TextMaterial")
text_obj.active_material.diffuse_color = (1, 1, 0, 1)  # Yellow color

bpy.ops.object.light_add(type='SUN', location=(10, -8, 10))
light = bpy.context.object
light.data.energy = 1.5

# Animate text scrolling and width change
for frame in range(scene.frame_start, scene.frame_end + 1):
    text_obj.location[2] += 0.1  # Move up
    text_obj.scale[0] -= 0.004  # Decrease width
    #if frame < 150:
    #    text_obj.location[2] += 0.1  # Move up
    #    text_obj.scale[0] -= 0.004  # Decrease width
    #else:
    #    text_obj.active_material.diffuse_color[3] -= 0.01  # Fade out transparency
    if frame > 180:
        text_obj.active_material.diffuse_color[3] -= 0.01  # Fade out transparency

    text_obj.keyframe_insert(data_path="location", frame=frame)
    text_obj.keyframe_insert(data_path="scale", frame=frame)
    text_obj.active_material.keyframe_insert(data_path="diffuse_color", frame=frame)

# Set up camera and render settings
camera_data = bpy.data.cameras.new(name='Camera')
camera_object = bpy.data.objects.new('Camera', camera_data)
scene.collection.objects.link(camera_object)
scene.camera = camera_object
camera_object.location = (0, -15, 0)
camera_object.rotation_euler = (1.1, 0, 0)

# Render settings
bpy.context.scene.render.engine = 'CYCLES'
bpy.context.scene.render.resolution_x = 200
bpy.context.scene.render.resolution_y = 160
bpy.context.scene.render.filepath = "/tmp/starwars"
bpy.context.scene.render.fps = 60

bpy.ops.render.render(animation=True)
