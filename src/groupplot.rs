use crate::axis::{Axis, AxisKey, AxisLike};
use crate::groupplot::GroupDimension::Rectangle;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Copy)]
pub enum GroupDimension {
    Horizontal(usize),
    Vertical(usize),
    Rectangle(usize, usize),
}

#[derive(Clone, Debug)]
pub struct GroupPlot {
    keys: Vec<AxisKey>,
    pub groups: Vec<Axis>,
    pub dimension: GroupDimension,
}

impl GroupPlot {
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

impl Default for GroupPlot {
    fn default() -> Self {
        Self {
            keys: vec![],
            groups: vec![],
            dimension: Rectangle(1, 1),
        }
    }
}

impl Display for GroupPlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\begin{{groupplot}}")?;
        match self.dimension {
            GroupDimension::Horizontal(n) => {
                writeln!(f, "[group style={{group size={} by 1}},", n)?;
            }
            GroupDimension::Vertical(n) => {
                writeln!(f, "[group style={{group size=1 by {}}},", n)?;
            }
            GroupDimension::Rectangle(m, n) => {
                writeln!(f, "[group style={{group size={} by {}}},", m, n)?;
            }
        }
        if !self.keys.is_empty() {
            for key in self.keys.iter() {
                writeln!(f, "\t{key},")?;
            }
        }
        writeln!(f, "]")?;
        for plot in self.groups.iter() {
            if !plot.keys.is_empty() {
                writeln!(f, "\\nextgroupplot[")?;
                for key in plot.keys.iter() {
                    writeln!(f, "\t{key},")?;
                }
                write!(f, "]")?;
            } else {
                writeln!(f, "\\nextgroupplot")?;
            }
            writeln!(f)?;
            for plot in plot.plots.iter() {
                writeln!(f, "{plot}")?;
            }
        }
        write!(f, "\\end{{groupplot}}")
    }
}

impl AxisLike for GroupPlot {
    fn needed_preamble(&self) -> Vec<String> {
        vec![String::from("\\usepgfplotslibrary{groupplots}")]
    }
}
