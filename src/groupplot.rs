use std::fmt::{Display, Formatter};
use crate::axis::{Axis, AxisKey, AxisLike};

#[derive(Clone, Debug)]
pub struct GroupPlot<const M: usize, const N: usize> {
    keys: Vec<AxisKey>,
    pub groups: [[Axis; N]; M],
}

impl<const M: usize, const N: usize> Default for GroupPlot<M, N> {
    fn default() -> Self {
        let d1: [Axis; N] = array_init::array_init(|_|{Default::default()});
        let groups: [[Axis; N]; M] = array_init::array_init(|_|{d1.clone()});
        Self{
            keys: vec![],
            groups,
        }
    }
}

impl<const M: usize, const N: usize> Display for GroupPlot<M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\begin{{groupplot}}")?;
        if !self.keys.is_empty() {
            writeln!(f, "[group size={} by {},", M, N)?;
            for key in self.keys.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;
        for i in 0..M {
            for j in 0..N {
                let axis = &self.groups[i][j];
                if !axis.keys.is_empty() {
                    writeln!(f, "\\nextgroupplot[")?;
                    for key in axis.keys.iter() {
                        writeln!(f, "\t{key},")?;
                    }
                    write!(f, "]")?;
                } else {
                    writeln!(f, "\\nextgroupplot")?;
                }
                writeln!(f)?;
                for plot in axis.plots.iter() {
                    writeln!(f, "{plot}")?;
                }
            }
        }
        write!(f, "\\end{{groupplot}}")
    }
}

impl<const M: usize, const N: usize> AxisLike  for GroupPlot<M, N> {
    fn needed_preamble(&self) -> Vec<String> {
        vec![String::from("\\usepgfplotslibrary{groupplots}")]
    }
}