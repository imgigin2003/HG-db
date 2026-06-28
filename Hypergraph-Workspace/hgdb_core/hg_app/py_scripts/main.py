import streamlit as st
from app_manager import AppManager

# Must run before any other Streamlit command.
st.set_page_config(
    page_title="HG-DB — Hypergraph Explorer",
    page_icon="🕸️",
    layout="wide",
    initial_sidebar_state="expanded",
)

# Light theming on top of the Streamlit theme (see .streamlit/config.toml).
st.markdown(
    """
    <style>
      .stButton > button {
        border-radius: 10px;
        border: 1px solid #A594F9;
        font-weight: 600;
      }
      .stButton > button:hover {
        border-color: #7E60BF;
        color: #7E60BF;
      }
      [data-testid="stMetric"] {
        background: #f6f3ff;
        border: 1px solid #e6def9;
        border-radius: 12px;
        padding: 12px 16px;
      }
      h1, h2, h3 { color: #4b3a78; }
    </style>
    """,
    unsafe_allow_html=True,
)


def main():
    app = AppManager()
    app.run()


if __name__ == "__main__":
    main()
