# select build image
FROM rust:1.30.1 as build

# create a new empty shell project
RUN USER=root cargo new --bin adventofcode2018_day1
WORKDIR /adventofcode2018_day1

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
# RUN cargo build --release
RUN cargo build
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/adventofcode2018_day1*
# RUN cargo build --release
RUN cargo build

# our final base
FROM rust:1.30.1

# copy the build artifact from the build stage
COPY --from=build /my_project/target/release/adventofcode2018_day1 .

# set the startup command to run your binary
CMD ["./adventofcode2018_day1"]