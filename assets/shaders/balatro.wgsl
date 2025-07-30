#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// Wild parameters for chaos
const PIXEL_SIZE_FAC: f32 = 800.0;  // More pixelated
const FLOW_EASE: f32 = 0.8;

// Primary color scheme (vibrant neon)
const COLOUR_1: vec4<f32> = vec4<f32>(1.0, 0.1, 0.8, 1.0);     // Hot pink
const COLOUR_2: vec4<f32> = vec4<f32>(0.1, 1.0, 0.3, 1.0);     // Lime green
const COLOUR_3: vec4<f32> = vec4<f32>(0.2, 0.4, 1.0, 1.0);     // Electric blue
const COLOUR_4: vec4<f32> = vec4<f32>(1.0, 0.6, 0.0, 1.0);     // Orange
const COLOUR_5: vec4<f32> = vec4<f32>(0.8, 0.0, 1.0, 1.0);     // Purple

// Alternative color scheme (dark cyberpunk)
const ALT_COLOUR_1: vec4<f32> = vec4<f32>(0.1, 0.8, 0.9, 1.0); // Cyan
const ALT_COLOUR_2: vec4<f32> = vec4<f32>(0.9, 0.1, 0.4, 1.0); // Deep red
const ALT_COLOUR_3: vec4<f32> = vec4<f32>(0.2, 0.2, 0.2, 1.0); // Dark gray
const ALT_COLOUR_4: vec4<f32> = vec4<f32>(1.0, 1.0, 0.1, 1.0); // Bright yellow
const ALT_COLOUR_5: vec4<f32> = vec4<f32>(0.4, 0.0, 0.8, 1.0); // Dark purple

// Third color scheme (fire/lava)
const FIRE_COLOUR_1: vec4<f32> = vec4<f32>(1.0, 0.3, 0.0, 1.0); // Red-orange
const FIRE_COLOUR_2: vec4<f32> = vec4<f32>(1.0, 0.8, 0.0, 1.0); // Yellow
const FIRE_COLOUR_3: vec4<f32> = vec4<f32>(0.8, 0.0, 0.0, 1.0); // Deep red
const FIRE_COLOUR_4: vec4<f32> = vec4<f32>(0.3, 0.0, 0.0, 1.0); // Dark red
const FIRE_COLOUR_5: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0); // White hot
const TWIST_AMOUNT: f32 = 2.1;
const CONTRAST: f32 = 1.8;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let screen_size = vec2<f32>(1.0, 1.0);
    let pixel_size = length(screen_size) / PIXEL_SIZE_FAC;

    let frag_coord = in.uv * screen_size;
    let quantized_uv = floor(frag_coord / pixel_size) * pixel_size;
    var uv = (quantized_uv - 0.5 * screen_size) / length(screen_size);

    let original_uv = uv;  // Keep original for effects
    let uv_len = length(uv);

    // Multi-layered rotation madness
    let time1 = globals.time * 0.3;
    let time2 = globals.time * 0.7;
    let time3 = globals.time * 1.1;

    // First spiral layer
    let base_angle = atan2(uv.y, uv.x);
    let angle_offset1 = sin(time1) * 10.0 * uv_len;
    let new_angle1 = base_angle + time1 + angle_offset1;

    let mid = (screen_size / length(screen_size)) * 0.5;
    uv = vec2<f32>(
        uv_len * cos(new_angle1) + mid.x,
        uv_len * sin(new_angle1) + mid.y
    ) - mid;

    // Second counter-rotating layer
    let angle_offset2 = cos(time2 * 1.5) * 8.0 * (1.0 - uv_len);
    let new_angle2 = base_angle - time2 * 1.3 + angle_offset2;
    var uv_layer2 = vec2<f32>(
        uv_len * cos(new_angle2),
        uv_len * sin(new_angle2)
    );

    // Kaleidoscope effect
    let kaleid_segments = 6.0;
    let kaleid_angle = atan2(original_uv.y, original_uv.x);
    let segment_angle = fract(kaleid_angle / (3.14159 * 2.0) * kaleid_segments) * 2.0 - 1.0;
    let kaleid_uv = vec2<f32>(segment_angle, uv_len) * 15.0;

    // Fractal noise layers
    uv *= 20.0;
    uv_layer2 *= 25.0;

    var noise_uv = uv + kaleid_uv * 0.3;
    var fractal_sum = 0.0;
    var amplitude = 1.0;

    // Multi-octave fractal
    for(var i: i32 = 0; i < 4; i++) {
        let t = time3 * (0.5 + f32(i) * 0.3);
        noise_uv += amplitude * vec2<f32>(
            sin(noise_uv.x * 0.7 + noise_uv.y * 0.3 + t),
            cos(noise_uv.y * 0.8 - noise_uv.x * 0.4 + t * 1.3)
        );

        fractal_sum += amplitude * sin(length(noise_uv) * 2.0 + t);
        amplitude *= 0.6;
        noise_uv *= 1.8;
    }

    // Plasma effect
    let plasma = sin(uv.x * 0.5 + time1) +
                cos(uv.y * 0.7 + time2) +
                sin((uv.x + uv.y) * 0.3 + time3) +
                cos(length(uv_layer2) * 0.4 + time1 * 2.0);

    // Wavy distortion
    let wave_x = sin(original_uv.y * 20.0 + time1 * 3.0) * 0.1;
    let wave_y = cos(original_uv.x * 15.0 + time2 * 2.0) * 0.1;
    let wave_uv = original_uv + vec2<f32>(wave_x, wave_y);

    // Multiple color zones
    let zone1 = sin(fractal_sum + plasma * 0.5) * 0.5 + 0.5;
    let zone2 = cos(length(uv_layer2) * 0.1 + time2) * 0.5 + 0.5;
    let zone3 = sin(wave_uv.x * wave_uv.y * 50.0 + time3) * 0.5 + 0.5;
    let zone4 = cos(atan2(wave_uv.y, wave_uv.x) * 3.0 + time1) * 0.5 + 0.5;
    let zone5 = sin(sqrt(abs(original_uv.x * original_uv.y)) * 30.0 + time2 * 1.5) * 0.5 + 0.5;

    // Chromatic aberration effect
    let aberration = 0.02 * sin(time1);
    let r_uv = original_uv + vec2<f32>(aberration, 0.0);
    let g_uv = original_uv;
    let b_uv = original_uv - vec2<f32>(aberration, 0.0);

    // Color scheme switching logic
    let scheme_cycle = globals.time * 0.15; // Slow cycle through schemes
    let scheme_id = floor(scheme_cycle) % 3.0;
    let flash_trigger = sin(globals.time * 0.8) > 0.95; // Random flashes
    let flash_intensity = select(0.0, 2.0, flash_trigger);

    // Choose color palette based on time
    var c1: vec4<f32>;
    var c2: vec4<f32>;
    var c3: vec4<f32>;
    var c4: vec4<f32>;
    var c5: vec4<f32>;

    if (scheme_id < 1.0) {
        // Primary neon scheme
        c1 = COLOUR_1;
        c2 = COLOUR_2;
        c3 = COLOUR_3;
        c4 = COLOUR_4;
        c5 = COLOUR_5;
    } else if (scheme_id < 2.0) {
        // Cyberpunk scheme
        c1 = ALT_COLOUR_1;
        c2 = ALT_COLOUR_2;
        c3 = ALT_COLOUR_3;
        c4 = ALT_COLOUR_4;
        c5 = ALT_COLOUR_5;
    } else {
        // Fire scheme
        c1 = FIRE_COLOUR_1;
        c2 = FIRE_COLOUR_2;
        c3 = FIRE_COLOUR_3;
        c4 = FIRE_COLOUR_4;
        c5 = FIRE_COLOUR_5;
    }

    // Color mixing with interference patterns
    let interference = sin(zone1 * 10.0) * sin(zone2 * 8.0) * sin(zone3 * 12.0);

    var final_color = c1 * zone1 * (1.0 + interference * 0.3) +
                      c2 * zone2 * (1.0 - interference * 0.2) +
                      c3 * zone3 * (1.0 + interference * 0.4) +
                      c4 * zone4 * (1.0 - interference * 0.1) +
                      c5 * zone5 * (1.0 + interference * 0.2);

    // Normalize and add dynamics
    final_color *= 0.3; // Tone down the intensity

    // Pulsing brightness
    let pulse = sin(time1 * 2.0) * 0.1 + 0.9;
    final_color *= pulse;

    // Add flash effect
    if (flash_trigger) {
        final_color = mix(final_color, vec4<f32>(1.0, 1.0, 1.0, 1.0), 0.7);
    }

    // Occasional color inversion flash
    let invert_flash = sin(globals.time * 0.3 + 1.5) > 0.98;
    if (invert_flash) {
        final_color = vec4<f32>(1.0, 1.0, 1.0, 1.0) - final_color;
        final_color.a = 1.0;
    }

    // Add some sparkle/glitch
    let glitch = step(0.98, sin(original_uv.x * 100.0 + time3) * cos(original_uv.y * 100.0 + time1));
    final_color += vec4<f32>(glitch, glitch, glitch, 0.0) * 0.5;

    // Vignette effect
    let vignette = 1.0 - length(original_uv) * 0.8;
    final_color *= max(vignette, 0.2);

    return clamp(final_color, vec4<f32>(0.0, 0.0, 0.0, 0.0), vec4<f32>(1.0, 1.0, 1.0, 1.0));
}
