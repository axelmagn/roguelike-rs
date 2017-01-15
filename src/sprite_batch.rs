use graphics::types::{Color, Rectangle, SourceRectangle};
use graphics::{ImageSize};
use std::collections::HashMap;

type Batch<T: ImageSize> =  HashMap<
                                Rc<T>, 
                                HashMap<
                                    Color, 
                                    Vec<(
                                        Rectangle, 
                                        SourceRectangle
                                    )>
                                >
                            >
