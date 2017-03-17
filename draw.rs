use gmatrix::Gmatrix;
use gmatrix::get_bezier;
use gmatrix::get_hermite;

use display::plot;

fn line1(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line1(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a+b;
	while x < x1 {
		plot(x,y, screen, color);
		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
}

fn line2(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*b+a;
	while y < y1 {
		plot(x,y, screen,color);
		if d<0 {
			x += 1;
			d += a;
		}
		y += 1;
		d += b;
	}
}

fn line7(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = a-(2*b);
	while y > y1 {
		plot(x,y, screen,color);
		if d>0 { //bc deltay = A = negative
			x += 1;
			d += a;
		}
		y -= 1;
		d -= b;
	}
}


fn line8(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0 as i32;
	let mut y = y0 as i32;
	if x0>x1 { return line8(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a-b;
	while x < x1 {
		plot(x,y,screen,color);
		if d<0 {
			y -= 1;
			d -= b;
		}
		x += 1;
		d += a;
	}
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let dx: f64 = (x1 as f64)-(x0 as f64) as f64;
	let dy: f64 = (y1 as f64)-(y0 as f64) as f64;
	if dx<0.0 { draw_line(x1,y1,x0,y0,screen,color); }

	let m = dy/dx;

	if (dy==0.0) && (dx==0.0) { return ; }
	if (m >= 0.0) && (m < 1.0) { line1(x0,y0,x1,y1,screen,color); } 
	else if m>=1.0 { line2(x0,y0,x1,y1,screen,color); } 
	else if (m <= 0.0) && (m > -1.0) { line8(x0,y0,x1,y1,screen,color); } 
	else if m<=-1.0 { line7(x0,y0,x1,y1,screen,color); } 
	else { println!("Should never reach this"); }
}

pub fn draw_lines(gm: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut i = 0;
	if gm.clen()<1 {
		return;
	}
	while i<gm.clen()-1 {
		draw_line(
			gm.get_val(0,i) as i32, //x0 
			gm.get_val(1,i) as i32, 
			gm.get_val(0,i+1) as i32, //y0 
			gm.get_val(1,i+1) as i32,
			screen,
			color);
		i += 2;
	}
}

fn circle_x(t: f32) -> f32 {
	return 250.0+100.0*t.cos();
}

fn circle_y(t: f32) -> f32 {
	return 250.0+100.0*t.sin();
}

fn paramet(edges: &mut Gmatrix, fx: &Fn(f32) -> f32, fy: &Fn(f32) -> f32, step: f32) {
	println!("step {}", step);
	let mut t = 0.0;
	let mut x0 = -1.0;
	let mut y0 = -1.0;
	while t <= 1.001 {
		let x1 = fx(t);
		let y1 = fy(t);
		println!("x: {} y: {}", x1, y1);
		if t>0.00 {
			println!("Adding edge! t={}",t);
			edges.add_edge(x0 as i32,y0 as i32,0,x1 as i32,y1 as i32,0);
		}
		x0 = x1;
		y0 = y1;
		t += step;
	}
}

pub fn add_curve(edges: &mut Gmatrix, x0:f32,y0:f32,x1:f32,y1:f32,a5:f32,a6:f32,a7:f32,a8:f32,tp:&str) {
	let mut giv = Gmatrix::new();
	let c;
	giv.add_val(0,x0);
	giv.add_val(1,x1);
	if (tp=="b") {
		giv.add_val(2,a5);
		giv.add_val(3,a7);
		c = get_bezier(&giv);
	} else {
		giv.add_val(2,a6/a5);
		giv.add_val(3,a8/a7);
		c = get_hermite(&giv);
	}
	//giv.print();
	c.print();
	//paramet(edges, )
}

pub fn add_circle(edges: &mut Gmatrix, cx: f32, cy: f32, cz: f32, r: f32) {
	paramet(edges, &circle_x, &circle_y, 0.01);
}