macro_rules! impl_approx_eq {
    ($Self:ty, $T:ty, $( $( $field:tt ).+ ),* $(,)?) => {
        #[cfg(any(feature = "approx", test))]
        impl approx::AbsDiffEq for $Self {
            type Epsilon = <$T as approx::AbsDiffEq>::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                <$T as approx::AbsDiffEq>::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                $(
                    approx::AbsDiffEq::abs_diff_eq(&self.$( $field ).+, &other.$( $field ).+, epsilon)
                ) && *
            }
        }

        #[cfg(any(feature = "approx", test))]
        impl approx::RelativeEq for $Self
        {
            fn default_max_relative() -> Self::Epsilon {
                <$T as approx::RelativeEq>::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                $(
                    approx::RelativeEq::relative_eq(&self.$( $field ).+, &other. $( $field ).+, epsilon, max_relative)
                ) && *
            }
        }

        #[cfg(any(feature = "approx", test))]
        impl approx::UlpsEq for $Self
        {
            fn default_max_ulps() -> u32 {
                <$T as approx::UlpsEq>::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                $(
                    approx::UlpsEq::ulps_eq(&self.$( $field ).+, &other.$( $field ).+, epsilon, max_ulps)
                ) && *
            }
        }
    };
}

pub(crate) use impl_approx_eq;
