struct Matrix { data: Vec<Vec<f32>>, }

impl Matrix {
	pub fn new(r: usize) -> Matrix {
		let mut ret = Vec::new();
		for _ in 0..r { ret.push(Vec::new()); }
		Matrix { data: ret }
	}

	pub fn identity(&self) -> Matrix {
		let mut ret = Matrix::new(self.rlen());
		for i in 0..self.rlen() {
			for j in 0..self.rlen() {
				if j==i {
					ret.add_val(i,1.0);
				} else {
					ret.add_val(i,0.0);
				}
			}
		}
		return ret;
	}

	pub fn get_val(&self, r: usize, c: usize) -> f32 {
		return self.data[r as usize][c as usize];
	}

	pub fn set_val(&mut self, r: usize, c: usize, val: f32) {
		self.data[r][c] = val;
	}

	pub fn rlen(&self) -> usize {
		return self.data.len() as usize;
	}

	pub fn clen(&self) -> usize {
		return self.data[0].len() as usize ;
	}

	pub fn print(&self) {
		let mut fin = String::new();
		//let mat = &self.data;
		for i in 0..self.rlen() {
			for j in 0..self.clen() {
				fin.push_str(&(self.data[i][j].to_string() + "\t"));
			}
			fin.push_str(&"\n");
		}
		println!("{}", fin);
	}

	pub fn add_val(&mut self, r: usize, val: f32) -> bool {
		if r>=self.rlen() { return false; } 
		else {
			self.data[r].push(val);
			return true;
		}
	}

	pub fn m_mult(&self, o: &Matrix) -> Matrix {
		let mut ret = Matrix::new(self.rlen());

		if self.clen()!=o.rlen() { 
			println!("Dimensions don't fit mult qualifications");
			return ret;
		}

		let mut val: f32;
		for r in 0..self.rlen() {
			for c in 0..o.clen() {
				val = 0.0;
				for k in 0..self.clen() {
					//println!("self.get({},{}): {}",r,k,self.get_val(r,k));
					//println!("o.get({},{}): {}\n",k,c,self.get_val(k,c));
					val += self.get_val(r,k)*o.get_val(k,c);
				}
				ret.add_val(r,val);
			}
		}
		return ret;
	}

	pub fn s_mult(&self, s: f32) -> Matrix {
		let mut ret = Matrix::new(self.rlen());
		for r in 0..self.rlen() {
			for c in 0..self.clen() {
				ret.add_val(r,self.get_val(r,c)*s);
			}
		}
		return ret;
	}
}

pub struct Gmatrix { data: Matrix, }

impl Gmatrix {
	pub fn new() -> Gmatrix { Gmatrix { data: Matrix::new(4) } }

	pub fn get_val(&self, r: usize, c: usize) -> f32 { return self.data.get_val(r,c); }

	pub fn add_val(&mut self, r: usize, val: f32) -> bool { return self.data.add_val(r,val); }

	pub fn set_val(&mut self, r: usize, c: usize, val: f32) {
		self.data.set_val(r,c,val);
	}

	pub fn rlen(&self) -> usize { return self.data.rlen(); }

	pub fn clen(&self) -> usize { return self.data.clen(); }

	pub fn s_mult(&self, s: f32) -> Gmatrix {
		let mut r = Gmatrix::new();
		r.data = self.data.s_mult(s);
		return r;
	}

	pub fn m_mult(&self, o: Gmatrix) -> Gmatrix {
		let mut r = Gmatrix::new(); 
		r.data = self.data.m_mult(&o.data);
		return r;
	}

	pub fn edit_mult(&self, o: &mut Gmatrix) {
		o.data = self.data.m_mult(&o.data);
	}

	pub fn add_pt(&mut self, x0: i32, y0: i32, z0: i32) {
		self.add_val(0, x0 as f32);
		self.add_val(1, y0 as f32);
		self.add_val(2, z0 as f32);
		self.add_val(3,1.0);
	}

	pub fn add_edge(&mut self,x0:i32,y0:i32,z0:i32,x1:i32,y1:i32,z1:i32) {
		self.add_pt(x0, y0, z0);
		self.add_pt(x1, y1, z1);
	}

	pub fn print(&self) { self.data.print(); }

	pub fn identity(&self) -> Gmatrix {
		let mut r = Gmatrix::new();
		r.data = self.data.identity();
		return r;
	}

	pub fn make_trans(&self, x: f32, y: f32, z: f32) -> Gmatrix {
		let mut r = self.identity();
		let mut i = 0;
		r.set_val(i,self.rlen()-1,x);
		i+=1;
		r.set_val(i,self.rlen()-1,y);
		i+=1;
		r.set_val(i,self.rlen()-1,z);
		return r;
	}

	pub fn make_scale(&self, x: f32, y: f32, z: f32) -> Gmatrix {
		let mut r = self.identity();
		let mut i = 0;
		r.set_val(i,i,x);
		i += 1;
		r.set_val(i,i,y);
		i += 1;
		r.set_val(i,i,z);
		return r;
	}

	pub fn make_rotX(&self, theta: f32) -> Gmatrix {
		let mut r = self.identity();
		let ang = theta.to_radians();
		let sin = ang.sin();
		let cos = ang.cos();
		r.set_val(1,1,cos);
		r.set_val(1,2,sin*-1.0);
		r.set_val(2,1,sin);
		r.set_val(2,2,cos);
		return r;
	}

	pub fn make_rotY(&self, theta: f32) -> Gmatrix {
		let mut r = self.identity();
		let ang = theta.to_radians();
		let cos = ang.cos();
		let sin = ang.sin();
		r.set_val(0,0,cos);
		r.set_val(0,2,sin);
		r.set_val(2,0,sin*-1.0);
		r.set_val(2,2,cos);
		return r;
	}

	pub fn make_rotZ(&self, theta: f32) -> Gmatrix {
		let ang = theta.to_radians();
		let mut r = self.identity();
		let sin = ang.sin();
		let cos = ang.cos();
		r.set_val(0,0,cos);
		r.set_val(0,1,sin*-1.0);
		r.set_val(1,0,sin);
		r.set_val(1,1,cos);
		return r;
	}
}