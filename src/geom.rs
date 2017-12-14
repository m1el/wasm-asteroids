use ::math::Vec2D;
// From Real-Time Collision Detection by Christer Ericson,
// published by Morgan Kaufmann Publishers, © 2005 Elsevier Inc
// p.139, ClosestPtPointTriangle
fn closest_triangle_point(p: Vec2D, a: Vec2D, b: Vec2D, c: Vec2D) -> Vec2D {
    let ab = b - a;
    let ac = c - a;
    let bc = c - b;
    // Compute parametric position s for projection P’ of P on AB,
    // P’ = A + s*AB, s = snom/(snom+sdenom)
    let snom = (p - a).dot(ab);
    let sdenom = (p - b).dot(a - b);
    // Compute parametric position t for projection P’ of P on AC,
    // P’ = A + t*AC, s = tnom/(tnom+tdenom)
    let tnom = (p - a).dot(ac);
    let tdenom = (p - c).dot(a - c);
    if snom <= 0.0 && tnom <= 0.0 { return a; } // Vertex region early out
    // Compute parametric position u for projection P’ of P on BC,
    // P’ = B + u*BC, u = unom/(unom+udenom)
    let unom = (p - b).dot(bc);
    let udenom = (p - c).dot(b - c);
    if sdenom <= 0.0 && unom <= 0.0 { return b; } // Vertex region early out
    if tdenom <= 0.0 && udenom <= 0.0 { return b; } // Vertex region early out
    // P is outside (or on) AB if the triple scalar product [N PA PB] <= 0
    let n = (b - a).cross(c - a);
    let vc = n * (a - p).cross(b - p);
    // If P outside AB and within feature region of AB,
    // return projection of P onto AB
    if vc < 0.0 && snom >= 0.0 && sdenom >= 0.0 {
        return a + ab.scale(snom / (snom + sdenom));
    }
    // P is outside (or on) BC if the triple scalar product [N PB PC] <= 0
    let va = n * (b - p).dot(c - p);
    // If P outside BC and within feature region of BC,
    // return projection of P onto BC
    if va <= 0.0 && unom >= 0.0 && udenom >= 0.0 {
        return b + bc.scale(unom / (unom + udenom));
    }
    // P is outside (or on) CA if the triple scalar product [N PC PA] <= 0
    let vb = n * (c - p).cross(a - p);
    // If P outside CA and within feature region of CA,
    // return projection of P onto CA
    if vb <= 0.0 && tnom >= 0.0 && tdenom >= 0.0 {
        return a + ac.scale(tnom / (tnom + tdenom));
    }
    // P must project inside face region. Compute Q using barycentric coordinates
    let u = va / (va + vb + vc);
    let v = vb / (va + vb + vc);
    let w = 1.0 - u - v; // = vc / (va + vb + vc)
    return a.scale(u) + b.scale(v) + c.scale(w);
}

pub fn test_circle_triangle(center: Vec2D, radius: f64, a: Vec2D, b: Vec2D, c: Vec2D) -> bool {
    let diff = center - closest_triangle_point(center, a, b, c);
    return diff.dot(diff) <= radius * radius;
}

pub fn test_circle_point(center: Vec2D, radius: f64, point: Vec2D) -> bool {
    let diff = center - point;
    return diff.dot(diff) <= radius * radius
}
