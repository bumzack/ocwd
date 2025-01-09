import bpy
import random

# Clear existing objects
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)

# Create a new scene
scene = bpy.context.scene
scene.frame_start = 1
scene.frame_end = 250
scene.render.resolution_x = 1920
scene.render.resolution_y = 1080

# Add a camera
camera = bpy.data.objects.new('Camera', bpy.data.cameras.new(name='Camera'))
bpy.context.collection.objects.link(camera)
camera.location = (0, -10, 5)
camera.rotation_euler = (1.3, 0, 0)
scene.camera = camera

# Add a text object
text_data = bpy.data.curves.new(type="FONT", name="TextCurve")
text_obj = bpy.data.objects.new("TextObject", text_data)
bpy.context.collection.objects.link(text_obj)
text_obj.location = (0, 0, 5)  # Start position at the top

# Set text properties
text_data.body = "A long time ago in a galaxy far,\nfar away....\n\nIt is a period of civil war.\nRebel spaceships, striking\nfrom a hidden base, have won\ntheir first victory against\nthe evil Galactic Empire."
text_data.size = 0.8
text_data.align_x = 'CENTER'
text_data.align_y = 'TOP'
text_data.extrude = 0.1

# Set material for the text (yellow with transparency)
material = bpy.data.materials.new(name="YellowText")
material.use_nodes = True
nodes = material.node_tree.nodes
links = material.node_tree.links

# Clear default nodes
for node in nodes:
    nodes.remove(node)

# Add new nodes
output_node = nodes.new(type='ShaderNodeOutputMaterial')
emission_node = nodes.new(type='ShaderNodeEmission')
emission_node.inputs['Color'].default_value = (1, 1, 0, 1)  # Yellow color
transparency_node = nodes.new(type='ShaderNodeBsdfTransparent')
mix_shader_node = nodes.new(type='ShaderNodeMixShader')

# Link nodes
links.new(emission_node.outputs['Emission'], mix_shader_node.inputs[1])
links.new(transparency_node.outputs['BSDF'], mix_shader_node.inputs[2])
links.new(mix_shader_node.outputs['Shader'], output_node.inputs['Surface'])

# Add a driver for transparency based on Z position
driver = mix_shader_node.inputs[0].driver_add('default_value').driver
var = driver.variables.new()
var.name = "z_pos"
var.targets[0].id_type = 'OBJECT'
var.targets[0].id = text_obj
var.targets[0].data_path = "location[2]"
driver.expression = "1 if z_pos > 8 else (z_pos - 8) / 4 + 0.5"

# Assign material to the text object
text_obj.data.materials.append(material)

# Add keyframes for scrolling and scaling
text_obj.keyframe_insert(data_path="location", frame=1)
text_obj.location.z = -5
text_obj.scale.x = 0.8
text_obj.scale.y = 0.8
text_obj.keyframe_insert(data_path="location", frame=250)

# Add keyframes for scaling
scale_keyframe_1 = text_obj.keyframe_insert(data_path="scale", frame=1)
scale_keyframe_1.interpolation = 'LINEAR'
scale_keyframe_2 = text_obj.keyframe_insert(data_path="scale", frame=250)
scale_keyframe_2.interpolation = 'LINEAR'

# Add a starry background
for _ in range(300):
    star = bpy.data.meshes.new('Star')
    star_obj = bpy.data.objects.new('Star', star)
    bpy.context.collection.objects.link(star_obj)
    star_obj.location = (random.uniform(-10, 10), random.uniform(-10, 10), random.uniform(0, 20))
    star_obj.scale = (0.05, 0.05, 0.05)

    # Add a material for the stars
    star_material = bpy.data.materials.new(name="StarMaterial")
    star_material.use_nodes = True
    nodes = star_material.node_tree.nodes
    links = star_material.node_tree.links

    # Clear default nodes
    for node in nodes:
        nodes.remove(node)

    # Add new nodes
    output_node = nodes.new(type='ShaderNodeOutputMaterial')
    emission_node = nodes.new(type='ShaderNodeEmission')
    emission_node.inputs['Color'].default_value = (1, 1, 1, 1)  # White color
    links.new(emission_node.outputs['Emission'], output_node.inputs['Surface'])

    star_obj.data.materials.append(star_material)

# Set the render engine to Eevee for real-time preview
scene.render.engine = 'BLENDER_EEVEE'

print("Star Wars intro animation setup complete.")


