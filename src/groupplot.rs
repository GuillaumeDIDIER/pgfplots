use std::fmt::{Display, Formatter};
use crate::axis::{Axis, AxisKey, AxisLike};

#[derive(Clone, Debug)]
pub struct GroupPlot<const M: usize, const N: usize> {
    keys: Vec<AxisKey>,
    pub groups: [[Axis; N]; M],
}

impl<const M: usize, const N: usize> GroupPlot<M,N>  {
    /// Creates a new, empty axis environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::groupplot::GroupPlot;
    ///
    /// let axis = GroupPlot::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the title of the axis environment. This can be valid LaTeX e.g.
    /// inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::groupplot::GroupPlot;
    ///
    /// let mut axis = GroupPlot::new();
    /// axis.set_title("My plot: $y = x^2$");
    /// ```
    pub fn set_title<S: Into<String>>(&mut self, title: S) {
        self.add_key(AxisKey::Title(title.into()));
    }
    /// Set the label of the *x* axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::groupplot::GroupPlot;
    ///
    /// let mut axis = GroupPlot::new();
    /// axis.set_x_label("$x$~[m]");
    /// ```
    pub fn set_x_label<S: Into<String>>(&mut self, label: S) {
        self.add_key(AxisKey::XLabel(label.into()));
    }
    /// Set the label of the *y* axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::groupplot::GroupPlot;
    ///
    /// let mut axis = GroupPlot::new();
    /// axis.set_y_label("$y$~[m]");
    /// ```
    pub fn set_y_label<S: Into<String>>(&mut self, label: S) {
        self.add_key(AxisKey::YLabel(label.into()));
    }
    /// Add a key to control the appearance of the axis. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::{AxisKey, Scale::Log};
    /// use pgfplots::groupplot::GroupPlot;
    /// let mut axis = GroupPlot::new();
    /// axis.add_key(AxisKey::YMode(Log));
    /// ```
    pub fn add_key(&mut self, key: AxisKey) {
        match key {
            AxisKey::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .keys
                    .iter()
                    .position(|k| std::mem::discriminant(k) == std::mem::discriminant(&key))
                {
                    self.keys.remove(index);
                }
            }
        }
        self.keys.push(key);
    }

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
            writeln!(f, "[group style={{group size={} by {}}},", M, N)?;
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